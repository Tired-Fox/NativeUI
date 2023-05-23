use style::{Appearance, Dimensions};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, RECT, WPARAM},
        Graphics::Gdi::UpdateWindow,
        UI::{
            Controls::ShowScrollBar,
            WindowsAndMessaging::{CreateWindowExW, GetClientRect, WINDOW_EX_STYLE, WINDOW_STYLE},
        },
    },
};

use crate::core::{
    constants::{SB, SBS, WS},
    to_RECT
};

use super::{ProcResult, Proc};

use native_core::{Component, Rect, Renderable};

#[derive(Debug)]
pub struct ScrollBar {
    id: String,
    classes: Vec<String>,
    pub handle: HWND,
    pub rect: Rect,
    ns_rect: Rect,
    pub size: i32,
    pub style: (Dimensions, Appearance),
    direction: i32,
}

impl Default for ScrollBar {
    fn default() -> Self {
        ScrollBar::new(0, SBS::VERT)
    }
}

impl ScrollBar {
    pub fn new(size: i32, direction: i32) -> Self {
        ScrollBar {
            handle: HWND(0),
            id: String::new(),
            classes: vec![String::from("scrollbar")],
            rect: Rect::new(0, 0, 0, 0),
            ns_rect: Rect::new(0, 0, 0, 0),
            size,
            direction,
            style: (Dimensions::default(), Appearance::default()),
        }
    }

    pub fn create(&mut self, parent: (HWND, HMODULE)) -> Result<(), String> {
        let mut rect: RECT = to_RECT(Rect::new(0, 0, 0, 0));
        let (handle, instance) = parent;
        unsafe {
            GetClientRect(handle, &mut rect as *mut RECT);
        }

        let (direction, rect) = match self.direction {
            SBS::HORZ => (
                WINDOW_STYLE(u32::try_from(self.direction).unwrap()),
                Rect::new(
                    rect.left,
                    rect.bottom - self.size,
                    rect.right - self.size,
                    rect.bottom,
                ),
            ),
            SBS::VERT => (
                WINDOW_STYLE(u32::try_from(self.direction).unwrap()),
                Rect::new(
                    rect.right - self.size,
                    rect.top,
                    rect.right,
                    rect.bottom - self.size,
                ),
            ),
            _ => (WINDOW_STYLE(0), Rect::new(0, 0, 0, 0)),
        };

        self.rect = rect;

        unsafe {
            self.handle = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PCWSTR(HSTRING::from("SCROLLBAR").as_ptr()),
                PCWSTR::null(),
                WS::CHILD | direction,
                self.rect.left,
                self.rect.top,
                self.rect.width(),
                self.rect.height(),
                handle,
                None,
                instance,
                None,
            );
        }
        assert!(self.handle.0 != 0, "Failed to create scrollbar");

        Ok(())
    }

    fn proc(&mut self, _hwnd: HWND, _msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        ProcResult::Default
    }
}

impl Proc for ScrollBar {
    fn proc(&mut self, hwnd: HWND, msg: u32, _wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        ProcResult::Default        
    }
}

impl Component for ScrollBar {}

impl Renderable for ScrollBar {
    fn update(&mut self) {
        unsafe { UpdateWindow(self.handle); }
    }

    fn show(&mut self) {
        unsafe {
            ShowScrollBar(self.handle, SB::CTL, true);
        }
    }

    fn hide(&mut self) {
        unsafe {
            ShowScrollBar(self.handle, SB::CTL, false);
        }
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn classes(&self) -> &Vec<String> {
        &self.classes
    }

    fn update_rect(&mut self, rect: Rect) {
        self.rect = rect
    }

    fn default_rect(&self) -> &Rect {
        &self.rect
    }
}
