use style::{Appearance, Dimensions, Unit};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, DrawTextW, EndPaint, GetDC, SetBkMode, PAINTSTRUCT, TRANSPARENT,
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, GetClientRect, SendMessageW, SetWindowLongPtrW, SetWindowPos,
            ShowWindow, GWL_WNDPROC, SWP_SHOWWINDOW, SW_HIDE, SW_SHOW, WM_CREATE,
        },
    },
};

use crate::core::{
    constants::{DT, WM, WS},
    to_RECT,
};

use native_core::{Component, Rect, Renderable};

use super::{
    helpers::{padding_rect, text_size},
    wndproc, Proc, ProcResult,
};

pub struct TextBuilder {
    pub text: HSTRING,
    pub rect: Rect,
    pub id: String,
    pub classes: Vec<String>,
}

impl TextBuilder {
    pub fn new(text: &str) -> TextBuilder {
        TextBuilder {
            text: HSTRING::from(text),
            rect: Rect::default(),
            id: String::new(),
            classes: vec![String::from("text")],
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = HSTRING::from(text);
        self
    }

    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.rect.right = width;
        self.rect.bottom = height;
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = String::from(id);
        self
    }

    pub fn classes(mut self, classes: Vec<String>) -> Self {
        self.classes.extend(classes.iter().map(|c| c.to_string()).collect::<Vec<String>>());
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.classes.push(String::from(class));
        self
    }

    pub fn build(self) -> Text {
        Text {
            handle: HWND(0),
            text: self.text,
            rect: self.rect,
            default_rect: Rect::default(),
            style: (Dimensions::default(), Appearance::default()),
            id: self.id,
            classes: self.classes,
            initialized: false,
        }
    }
}

#[derive(Debug)]
pub struct Text {
    pub handle: HWND,
    pub text: HSTRING,
    pub rect: Rect,
    pub default_rect: Rect,
    pub style: (Dimensions, Appearance),
    pub id: String,
    pub classes: Vec<String>,
    pub initialized: bool,
}

impl Proc for Text {
    fn proc(&mut self, hwnd: HWND, msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        match msg {
            WM::PAINT => unsafe {
                let mut rect: RECT = to_RECT(self.rect);
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
}

impl Component for Text {}

impl Text {
    pub fn new(text: &str) -> Self {
        Text {
            handle: HWND(0),
            text: HSTRING::from(text),
            rect: Rect::default(),
            default_rect: Rect::default(),
            style: (Dimensions::default(), Appearance::default()),
            id: String::new(),
            classes: vec![String::from("text")],
            initialized: false,
        }
    }

    pub fn builder(text: &str) -> TextBuilder {
        TextBuilder::new(text)
    }

    fn create(&mut self, parent: (HWND, HMODULE)) -> Result<(), String> {
        if !self.initialized {
            let (handle, instance) = parent;
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

            self.default_rect = text_size(self.handle, self.text.to_string_lossy());
            match self.style.0.width {
                Unit::PX(width) => self.rect.right = width as i32,
                _ => self.rect.right = self.default_rect.right,
            }
            match self.style.0.width {
                Unit::PX(height) => self.rect.bottom = height as i32,
                _ => self.rect.bottom = self.default_rect.bottom,
            }

            self.update();
            self.initialized = true;
        }
        Ok(())
    }
}

impl Renderable for Text {
    fn show(&mut self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    fn hide(&mut self) {
        unsafe {
            ShowWindow(self.handle, SW_HIDE);
        }
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn update(&mut self) {
        unsafe {
            SetWindowPos(
                self.handle,
                None,
                self.rect.left,
                self.rect.top,
                self.rect.width(),
                self.rect.height(),
                SWP_SHOWWINDOW,
            );
        }
    }

    fn default_rect(&self) -> &Rect {
        &self.default_rect
    }

    fn classes(&self) -> &Vec<String> {
        &self.classes
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn update_rect(&mut self, rect: Rect) {
        self.rect = rect
    }
}
