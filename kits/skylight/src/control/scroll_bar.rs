use style::{Appearance, Dimensions, Stylesheet};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, RECT, WPARAM},
        UI::{
            Controls::ShowScrollBar,
            WindowsAndMessaging::{CreateWindowExW, GetClientRect, WINDOW_EX_STYLE, WINDOW_STYLE, ShowWindow},
        },
    },
};

use crate::core::{
    constants::{SBS, WS, SB},
    ProcResult, Rect, Renderable, ViewType,
};

use super::Control;

#[derive(Debug)]
pub struct ScrollBar {
    parent: ViewType,

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
            parent: ViewType::None,
            handle: HWND(0),
            rect: Rect::new(0, 0, 0, 0),
            ns_rect: Rect::new(0, 0, 0, 0),
            size,
            direction,
            style: (Dimensions::default(), Appearance::default()),
        }
    }
}

impl Control for ScrollBar {
    fn create(&mut self, parent: ViewType, stylesheet: &Stylesheet) -> Result<(), String> {
        self.parent = parent;
        let mut rect: RECT = Rect::new(0, 0, 0, 0).into();
        let (handle, instance) = match self.parent {
            ViewType::Window(hwnd, instance) => unsafe {
                GetClientRect(hwnd, &mut rect as *mut RECT);
                (hwnd, instance)
            },
            _ => (HWND(0), HMODULE(0)),
        };

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
        println!("{:?}", self.rect);

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

    fn ns_rect(&self) -> &Rect {
        &self.ns_rect
    }

    fn proc(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> ProcResult {
        ProcResult::Default
    }

    fn classes(&mut self, classes: Vec<&'static str>) {}
}

impl Renderable for ScrollBar {
    fn update(
        &mut self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: Option<(Rect, (Dimensions, Appearance))>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn show(&self) {
        unsafe { ShowScrollBar(self.handle, SB::CTL, true); }
    }

    fn hide(&self) {
        unsafe { ShowScrollBar(self.handle, SB::CTL, false); }
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
