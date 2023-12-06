use std::cell::RefCell;

use windows::Win32::Foundation::{COLORREF, HWND};
use windows::Win32::UI::Controls::Dialogs::{
    ChooseColorW, CC_ANYCOLOR, CC_FULLOPEN, CC_PREVENTFULLOPEN, CC_RGBINIT, CHOOSECOLORW,
    CHOOSECOLOR_FLAGS,
};

use crate::error::Error;
use crate::modal::DialogAction;
use crate::windows::modal::get_dlg_error;
use crate::windows::swap_rb;

thread_local! {
    static CUSTOM_COLORS: RefCell<Vec<COLORREF>> = RefCell::new(Vec::new());
}

#[derive(Default, Debug, Clone)]
pub struct ColorPicker {
    initial_color: Option<u32>,
}

impl ColorPicker {
    pub fn get_custom_colors() -> Vec<u32> {
        CUSTOM_COLORS.with(|global_colors| {
            global_colors
                .borrow()
                .iter()
                .map(|v| swap_rb(v.0))
                .collect()
        })
    }

    pub fn set_custom_colors(colors: Vec<u32>) {
        CUSTOM_COLORS.with(|global_colors| {
            *global_colors.borrow_mut() = colors.iter().map(|v| COLORREF(swap_rb(*v))).collect();
        })
    }

    pub fn new(initial_color: Option<u32>) -> Self {
        Self {
            initial_color: initial_color.map(|v| swap_rb(v)),
            ..Default::default()
        }
    }

    pub fn show_with(&self, parent: isize) -> Result<DialogAction, Error> {
        CUSTOM_COLORS.with(|v| {
            let custom_colors = &mut (*v.borrow_mut());
            if custom_colors.len() < 16 {
                custom_colors.resize(16, COLORREF(0xFFFFFF));
            }

            unsafe {
                let mut options = CHOOSECOLORW {
                    hwndOwner: HWND(parent),
                    lStructSize: std::mem::size_of::<CHOOSECOLORW>() as u32,
                    rgbResult: COLORREF(self.initial_color.unwrap_or(0)),
                    Flags: self
                        .initial_color
                        .map_or(CHOOSECOLOR_FLAGS(0), |_| CC_RGBINIT)
                        | CC_FULLOPEN
                        | CC_PREVENTFULLOPEN
                        | CC_ANYCOLOR,
                    lpCustColors: custom_colors.get_mut(0).unwrap(),

                    ..Default::default()
                };

                if ChooseColorW(&mut options).into() {
                    Ok(DialogAction::Color(swap_rb(options.rgbResult.0)))
                } else {
                    get_dlg_error()
                }
            }
        })
    }

    pub fn show(&self) -> Result<DialogAction, Error> {
        self.show_with(0)
    }
}
