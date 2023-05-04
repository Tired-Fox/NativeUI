use windows::Win32::Foundation::{LRESULT, LPARAM, WPARAM, HWND};

use crate::core::{Renderable, ViewType, ProcResult};

use super::Control;


#[derive(Debug)]
pub struct Button {
    parent: ViewType
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
    fn update() -> Result<(), String> {
        Ok(())
    }

    fn show(&self) {
        
    }
}
