
use style::{Stylesheet, Dimensions, Appearance};
use windows::Win32::Foundation::{LPARAM, WPARAM, HWND};

use crate::core::{Renderable, ViewType, ProcResult, Rect};

use super::Control;


#[derive(Debug)]
pub struct Button {
    parent: ViewType,
    pub rect: Rect,
    pub style: (Dimensions, Appearance),
}

impl Control for Button {
    fn create(&mut self, parent: ViewType, stylesheet: &Stylesheet) -> Result<(), String> {
        self.style = stylesheet.get_styles(vec!["button".to_owned()]);

        self.parent = parent;
        Ok(())
    }

    fn proc(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> ProcResult {
        ProcResult::Default
    }

    fn classes(&mut self, classes: Vec<&'static str>) {
        
    }
}

impl Renderable for Button {
    fn update(
        &self,
        parent: (Rect, (Dimensions, Appearance)),
        previous: (Rect, (Dimensions, Appearance)),
    ) -> Result<(), String> {
        Ok(())
    }

    fn show(&self) {
        
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn style(&self) -> &(Dimensions, Appearance) { 
        &self.style
    }
}
