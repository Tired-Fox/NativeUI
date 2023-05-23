use style::{Appearance, Dimensions, Stylesheet};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};

use crate::{core::{Renderable, ViewType}, ui::component::ProcResult};

use native_core::Rect;

use super::Control;

#[derive(Debug)]
pub struct Button {
    parent: ViewType,

    pub handle: HWND,
    pub rect: Rect,
    ns_rect: Rect,
    pub style: (Dimensions, Appearance),
}

impl Control for Button {
    fn create(&mut self, parent: ViewType, stylesheet: &Stylesheet) -> Result<(), String> {
        self.style = stylesheet.get_styles(vec!["button".to_owned()]);

        self.parent = parent;
        self.handle = HWND(0);
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

impl Renderable for Button {
    fn update(
        &mut self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: Option<(Rect, (Dimensions, Appearance))>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn show(&self) {}

    fn hide(&self) {}

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
