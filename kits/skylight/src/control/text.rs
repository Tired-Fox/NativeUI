use style::{color::Color, Appearance, Dimensions, Position, Stylesheet, Unit};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{COLORREF, HWND, LPARAM, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateSolidBrush, DrawTextW, EndPaint, FrameRect, GetDC, SetBkMode,
            PAINTSTRUCT, TRANSPARENT,
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, GetClientRect, SendMessageW, SetWindowLongPtrW, ShowWindow,
            GWL_WNDPROC, SW_HIDE, SW_SHOW, WM_CREATE,
        },
    },
};

use crate::{
    control::helpers::update_pos,
    core::{
        constants::{DT, WM, WS},
        ProcResult, Rect, Renderable, ViewType,
    },
};

use super::{
    helpers::{padding_rect, text_size},
    wndproc, Control,
};

#[derive(Debug)]
pub struct Text {
    parent: ViewType,
    pub handle: HWND,
    pub text: HSTRING,
    pub rect: Rect,
    ns_rect: Rect,
    pub text_rect: Rect,
    pub style: (Dimensions, Appearance),
    pub classes: Vec<String>,
    pub initialized: bool,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Text {
            parent: ViewType::None,
            handle: HWND(0),
            text: HSTRING::from(text),
            rect: Rect::new(0, 0, 0, 0),
            ns_rect: Rect::new(0, 0, 0, 0),
            text_rect: Rect::new(0, 0, 0, 0),
            style: (Dimensions::default(), Appearance::default()),
            classes: Vec::new(),
            initialized: false,
        }
    }
}

impl Control for Text {
    fn classes(&mut self, classes: Vec<&'static str>) {
        self.classes = classes
            .iter()
            .map(|c| {
                if !c.starts_with(".") {
                    String::from(".") + c
                } else {
                    String::from(*c)
                }
            })
            .collect::<Vec<String>>();
    }

    fn ns_rect(&self) -> &Rect {
        &self.ns_rect
    }

    fn proc(&mut self, hwnd: HWND, msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        match msg {
            WM::PAINT => unsafe {
                let mut rect: RECT = self.rect.into();
                GetClientRect(hwnd, &mut rect as *mut RECT);

                padding_rect(self, &mut rect);

                if rect.right > rect.left && rect.bottom > rect.top {
                    let mut ps = PAINTSTRUCT {
                        hdc: GetDC(self.handle),
                        ..Default::default()
                    };
                    let hdc = BeginPaint(hwnd, &mut ps as *mut PAINTSTRUCT);

                    // FrameRect(
                    //     hdc,
                    //     &rect as *const RECT,
                    //     CreateSolidBrush(COLORREF(Color::new(0, 0, 0, 1.).into())),
                    // );

                    SetBkMode(hdc, TRANSPARENT);
                    let mut text: Vec<u16> = self.text.to_string_lossy().encode_utf16().collect();
                    DrawTextW(
                        hdc,
                        &mut text[..],
                        &mut rect as *mut RECT,
                        DT::CENTER | DT::SINGLELINE | DT::VCENTER,
                    );
                    EndPaint(self.handle, &mut ps as *mut PAINTSTRUCT);
                }

                ProcResult::Success
            },
            _ => ProcResult::Default,
        }
    }

    fn create(&mut self, parent: ViewType, stylesheet: &Stylesheet) -> Result<(), String> {
        let mut selectors: Vec<String> = vec!["text".to_owned()];
        selectors.extend(self.classes.clone());

        self.style = stylesheet.get_styles(selectors);

        if !self.initialized {
            self.parent = parent;

            let (handle, instance) = match &self.parent {
                ViewType::Window(handle, instance) if handle.0 != 0 && instance.0 != 0 => {
                    (handle, instance)
                }
                _ => return Err("Invalid control parent".to_owned()),
            };

            unsafe {
                self.handle = CreateWindowExW(
                    windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
                    PCWSTR(HSTRING::from("STATIC").as_ptr()),
                    PCWSTR(self.text.as_ptr()),
                    WS::VISIBLE | WS::CHILD | WS::BORDER,
                    self.rect.left,
                    self.rect.top,
                    self.rect.width(),
                    self.rect.height(),
                    handle.to_owned(),
                    None,
                    instance.to_owned(),
                    None,
                );

                SetWindowLongPtrW(self.handle, GWL_WNDPROC, wndproc::<Text> as isize);
                SendMessageW(
                    self.handle,
                    WM_CREATE,
                    WPARAM(0),
                    LPARAM(&self as *const _ as isize),
                );
            }

            assert!(self.handle.0 != 0);

            self.text_rect = text_size(self.handle, self.text.to_string_lossy());
            match self.style.0.width {
                Unit::PX(width) => self.rect.right = width as i32,
                _ => self.rect.right = self.text_rect.right,
            }
            match self.style.0.width {
                Unit::PX(height) => self.rect.bottom = height as i32,
                _ => self.rect.bottom = self.text_rect.bottom,
            }

            update_pos(self);
            self.initialized = true;
        }
        Ok(())
    }
}

impl Renderable for Text {
    fn update(
        &mut self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: Option<(Rect, (Dimensions, Appearance))>,
    ) -> Result<(), String> {
        // TODO: Implement inset alignment based ond display relative or absolute

        let mut add_padding = (false, false);
        let dimensions = self.style().0;

        let parent_padding = parent
            .1
             .0
            .padding
            .calc(parent.0.width(), parent.0.height());

        let inset = dimensions.inset.calc(parent.0.width(), parent.0.height());
        let margin = self
            .style()
            .0
            .margin
            .calc(parent.0.width(), parent.0.height());

        let width = dimensions.width.as_i32(
            parent.0.width() - parent_padding.1 - parent_padding.3 - margin.1 - margin.3,
            match dimensions.position {
                style::Position::Absolute
                    if inset.3 != 0 && inset.1 != 0 && dimensions.width == Unit::Default =>
                {
                    parent.0.width() - inset.3 - inset.1 - margin.1 - margin.3
                }
                _ => match dimensions.width {
                    Unit::FitConent => {
                        add_padding.0 = true;
                        self.text_rect.width()
                    }
                    _ => {
                        parent.0.width() - parent_padding.1 - parent_padding.3 - margin.1 - margin.3
                    }
                },
            },
        );

        let height = dimensions.height.as_i32(
            parent.0.height() - parent_padding.0 - parent_padding.2,
            match dimensions.position {
                Position::Absolute
                    if inset.0 != 0 && inset.2 != 0 && dimensions.height == Unit::Default =>
                {
                    parent.0.height() - inset.2 - inset.0 - margin.2 - margin.0
                }
                _ => match dimensions.height {
                    Unit::FitConent | Unit::Default => {
                        add_padding.1 = true;
                        self.text_rect.height()
                    }
                    _ => {
                        parent.0.height()
                            - margin.0
                            - margin.2
                            - parent_padding.0
                            - parent_padding.2
                    }
                },
            },
        );

        let padding = self.style().0.padding.calc(width, height);

        self.rect.left = match dimensions.position {
            style::Position::Absolute => match dimensions.inset.left {
                Unit::Default => match dimensions.inset.right {
                    Unit::Default => margin.3,
                    _ => parent.0.width() - margin.1 - inset.1 - width,
                },
                _ => margin.3 + inset.3,
            },
            _ => parent_padding.3 + margin.3,
        };

        self.rect.top = match dimensions.position {
            style::Position::Absolute => match dimensions.inset.top {
                Unit::Default => match dimensions.inset.bottom {
                    Unit::Default => margin.0,
                    _ => parent.0.height() - margin.2 - inset.2 - height,
                },
                _ => margin.0 + inset.0,
            },
            _ => match previous {
                Some(prev) => {
                    prev.0.bottom
                        + prev.1 .0.margin.bottom.as_i32(parent.0.height(), 0)
                        + parent_padding.0
                        + margin.0
                }
                None => parent_padding.0 + margin.0,
            },
        };

        self.rect.right = self.rect.left + width;
        self.rect.bottom = self.rect.top + height;

        if add_padding.0 {
            self.rect.right += padding.3 + padding.1 + 4;
        }

        if add_padding.1 {
            self.rect.bottom += padding.0 + padding.2;
        }

        match dimensions.max_width {
            Unit::Default => (),
            _ => {
                let max = dimensions.max_width.as_i32(parent.0.width(), 0);
                if self.rect.width() > max {
                    self.rect.right -= self.rect.width() - max;
                }
            }
        }

        match dimensions.min_width {
            Unit::Default => (),
            _ => {
                let min = dimensions.min_width.as_i32(parent.0.width(), 0);
                if self.rect.width() < min {
                    self.rect.right += min - self.rect.width();
                }
            }
        };

        match dimensions.max_height {
            Unit::Default => (),
            _ => {
                let max = dimensions.max_height.as_i32(parent.0.height(), 0);
                if self.rect.height() > max {
                    self.rect.bottom -= self.rect.height() - max;
                }
            }
        }

        match dimensions.min_height {
            Unit::Default => (),
            _ => {
                let min = dimensions.min_height.as_i32(parent.0.height(), 0);
                if self.rect.height() < min {
                    self.rect.bottom += min - self.rect.height();
                }
            }
        };

        self.ns_rect = self.rect.clone();
        self.ns_rect.top += padding.0;
        self.ns_rect.right -= padding.1;
        self.ns_rect.bottom -= padding.2;
        self.ns_rect.left += padding.3;
        update_pos(self);

        Ok(())
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    fn hide(&self) {
        unsafe {
            ShowWindow(self.handle, SW_HIDE);
        }
    }

    fn handle(&self) -> &HWND {
        &self.handle
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn style(&self) -> &(Dimensions, Appearance) {
        &self.style
    }
}
