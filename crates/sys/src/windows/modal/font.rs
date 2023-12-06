use crate::error::Error;
use crate::modal::{DialogAction, FontWeight};
use crate::windows::modal::{cwide_to_string, get_dlg_error};
use windows::core::PWSTR;
use windows::Win32::Foundation::{COLORREF, HWND};
use windows::Win32::Graphics::Gdi::{GetDC, GetDeviceCaps, LOGFONTW, LOGPIXELSY};
use windows::Win32::UI::Controls::Dialogs::{
    ChooseFontW, CF_EFFECTS, CF_INITTOLOGFONTSTRUCT, CF_SCREENFONTS, CF_USESTYLE, CHOOSEFONTW,
    REGULAR_FONTTYPE,
};

#[derive(Default, Debug, Clone)]
pub struct FontDialog {
    pub weight: FontWeight,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub point_size: u32,
}

impl From<i32> for FontWeight {
    fn from(v: i32) -> Self {
        match v {
            1..=100 => FontWeight::Thin,
            101..=200 => FontWeight::ExtraLight,
            201..=300 => FontWeight::Light,
            301..=400 => FontWeight::Regular,
            401..=500 => FontWeight::Medium,
            501..=600 => FontWeight::SemiBold,
            601..=700 => FontWeight::Bold,
            701..=800 => FontWeight::ExtraBold,
            801..=900 => FontWeight::Black,
            _ => FontWeight::Any,
        }
    }
}

impl FontDialog {
    pub fn show(&self, parent: isize) -> Result<DialogAction, Error> {
        let mut face_name = [0; 32];
        face_name[0] = 'A' as u16;
        face_name[1] = 'r' as u16;
        face_name[2] = 'i' as u16;
        face_name[3] = 'a' as u16;
        face_name[4] = 'l' as u16;

        unsafe {
            let mut font = LOGFONTW {
                lfHeight: (self.point_size as i32 * GetDeviceCaps(GetDC(None), LOGPIXELSY)) / 72,
                lfWeight: self.weight as i32,
                lfItalic: self.italic as u8,
                lfUnderline: self.underline as u8,
                lfStrikeOut: self.strikethrough as u8,
                lfFaceName: face_name,
                ..Default::default()
            };

            let mut style = {
                let items = String::from("Regular").encode_utf16().collect::<Vec<u16>>();
                let mut result = [0; 32];
                for (idx, i) in items.iter().enumerate() {
                    result[idx] = *i;
                }
                result
            };

            let mut cf: CHOOSEFONTW = CHOOSEFONTW {
                lStructSize: std::mem::size_of::<CHOOSEFONTW>() as u32,
                hwndOwner: HWND(parent),
                lpLogFont: &mut font,
                Flags: CF_SCREENFONTS | CF_EFFECTS | CF_INITTOLOGFONTSTRUCT | CF_USESTYLE,
                rgbColors: COLORREF(0), // Only if CF_INITTOLOGFONTSTRUCT is set
                nFontType: REGULAR_FONTTYPE,
                lpszStyle: PWSTR(style.as_mut_ptr()),
                ..Default::default()
            };

            if ChooseFontW(&mut cf).into() {
                Ok(DialogAction::Font {
                    name: cwide_to_string(&font.lfFaceName),
                    weight: FontWeight::from(font.lfWeight),
                    italic: font.lfItalic != 0,
                    underline: font.lfUnderline != 0,
                    strikethrough: font.lfStrikeOut != 0,
                    size: cf.iPointSize as u32 / 10,
                })
            } else {
                get_dlg_error()
            }
        }
    }
}
