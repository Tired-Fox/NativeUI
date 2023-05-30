use std::{collections::HashSet, ptr::hash};

use style::{Appearance, Dimensions};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HMODULE, HWND, RECT},
        Graphics::Gdi::UpdateWindow,
        UI::{
            Controls::ShowScrollBar,
            WindowsAndMessaging::{CreateWindowExW, GetClientRect, WINDOW_EX_STYLE, WINDOW_STYLE},
        },
    },
};

use crate::core::{
    constants::{SB, SBS, WS},
    to_RECT,
};

use super::Proc;

use native_core::{Component, Rect, Renderable};

#[derive(Debug)]
pub struct ScrollBar {
    id: String,
    classes: HashSet<String>,
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
            classes: HashSet::from(["scrollbar".to_string()]),
            rect: Rect::new(0, 0, 0, 0),
            ns_rect: Rect::new(0, 0, 0, 0),
            size,
            direction,
            style: (Dimensions::default(), Appearance::default()),
        }
    }
}

impl Component<(HWND, HMODULE)> for ScrollBar {
    fn create(&mut self, data: (HWND, HMODULE)) -> Result<(), String> {
        let mut rect: RECT = to_RECT(Rect::new(0, 0, 0, 0));
        let (handle, instance) = data;
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
}

impl Proc for ScrollBar {}

impl Renderable for ScrollBar {
    fn update(&mut self, rect: Rect) {
        unsafe {
            UpdateWindow(self.handle);
        }
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

    fn classes(&self) -> &HashSet<String> {
        &self.classes
    }

    fn default_rect(&self) -> &Rect {
        &self.rect
    }
}
