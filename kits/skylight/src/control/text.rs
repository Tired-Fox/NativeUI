use style::{Appearance, Dimensions, Stylesheet, Unit};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, DrawTextW, EndPaint, GetDC, SetBkMode, PAINTSTRUCT, TRANSPARENT,
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, GetClientRect, SendMessageW, SetWindowLongPtrW, SetWindowPos,
            ShowWindow, GWL_WNDPROC, SET_WINDOW_POS_FLAGS, SW_SHOW, WM_CREATE,
        },
    },
};

use crate::core::{
    constants::{DT, WM, WS},
    ProcResult, Rect, Renderable, ViewType,
};

use super::{helpers::text_size, wndproc, Control};

#[derive(Debug)]
pub struct Text {
    parent: ViewType,
    pub handle: HWND,
    pub text: HSTRING,
    pub rect: Rect,
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
            style: (Dimensions::default(), Appearance::default()),
            classes: Vec::new(),
            initialized: false,
        }
    }

}

impl Control for Text {
    fn classes(&mut self, classes: Vec<&'static str>) {
        println!("{:?}", classes);
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

    fn proc(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> ProcResult {
        match msg {
            WM::PAINT => unsafe {
                let mut rect = self.rect.into();
                GetClientRect(hwnd, &mut rect as *mut RECT);

                let mut ps = PAINTSTRUCT {
                    hdc: GetDC(self.handle),
                    ..Default::default()
                };
                let hdc = BeginPaint(hwnd, &mut ps as *mut PAINTSTRUCT);

                SetBkMode(hdc, TRANSPARENT);
                let mut text: Vec<u16> = self.text.to_string_lossy().encode_utf16().collect();
                DrawTextW(
                    hdc,
                    &mut text[..],
                    &mut self.rect.into() as *mut RECT,
                    DT::CENTER | DT::SINGLELINE | DT::VCENTER,
                );
                EndPaint(self.handle, &mut ps as *mut PAINTSTRUCT);

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
                    WS::VISIBLE | WS::CHILD,
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
            let text_rect = text_size(self.handle, self.text.to_string_lossy());
            match self.style.0.width {
                Unit::PX(width) => self.rect.right = width as i32,
                Unit::Percent(width) => self.rect.right = text_rect.right,
                Unit::Default => self.rect.right = text_rect.right,
            }
            match self.style.0.width {
                Unit::PX(height) => self.rect.bottom = height as i32,
                Unit::Percent(_) => self.rect.bottom = text_rect.bottom,
                Unit::Default => self.rect.bottom = text_rect.bottom,
            }

            unsafe {
                SetWindowPos(
                    self.handle,
                    None,
                    self.rect.left,
                    self.rect.top,
                    self.rect.width(),
                    self.rect.height(),
                    SET_WINDOW_POS_FLAGS::default(),
                );
            }
            self.initialized = true;
        }
        Ok(())
    }
}

impl Renderable for Text {
    fn update(
        &self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: (Rect, (Dimensions, Appearance)),
    ) -> Result<(), String> {
        println!("Update Text");
        Ok(())
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn style(&self) -> &(Dimensions, Appearance) {
        &self.style
    }
}
