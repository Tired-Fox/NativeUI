use std::collections::HashMap;

use style::styles::Prop;
use windows::Win32::Foundation::{LPARAM, WPARAM, HWND};

use crate::core::{Renderable, ViewType, ProcResult, Rect};

use super::Control;


#[derive(Debug)]
pub struct Button {
    parent: ViewType,
    pub rect: Rect,
    pub style: HashMap<String, Prop>,
}

impl Control for Button {
    fn create(&mut self, parent: crate::core::ViewType) -> Result<(), String> {
        self.parent = parent;
        Ok(())
    }

    fn proc(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> ProcResult {
        ProcResult::Default
    }
}

impl Renderable for Button {
    fn update(
        &self,
        parent: (Rect, HashMap<String, Prop>),
        previous: (Rect, HashMap<String, Prop>),
    ) -> Result<(), String> {
        Ok(())
    }

    fn show(&self) {
        
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn style(&self) -> &HashMap<String, Prop> { 
        &self.style
    }
}
