use windows::Win32::Foundation::{COLORREF, HWND};
use windows::Win32::Graphics::Gdi::LOGFONTW;
use windows::Win32::UI::Controls::Dialogs::{CF_SCREENFONTS, CHOOSEFONTW, CF_EFFECTS, CF_INITTOLOGFONTSTRUCT, REGULAR_FONTTYPE, ChooseFontW};
use crate::error::Error;
use crate::modal::{DialogAction, FontWeight};
use crate::windows::modal::{cwide_to_string, get_dlg_error};

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
            _ => FontWeight::Any

        }
    }
}

impl FontDialog {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn point_size(mut self, point_size: u32) -> Self {
        self.point_size = point_size;
        self
    }

    pub fn show(&self) -> Result<DialogAction, Error> {
        unsafe {
            let mut font = LOGFONTW {
                lfWeight: self.weight as i32,
                lfItalic: self.italic as u8,
                lfUnderline: self.underline as u8,
                lfStrikeOut: self.strikethrough as u8,
                lfFaceName: [0; 32],
                ..Default::default()
            };
            let mut CF: CHOOSEFONTW = CHOOSEFONTW {
                lStructSize: std::mem::size_of::<CHOOSEFONTW>() as u32,
                hwndOwner: HWND(0),
                lpLogFont: &mut font,
                iPointSize: self.point_size as i32,
                Flags: CF_SCREENFONTS | CF_EFFECTS | CF_INITTOLOGFONTSTRUCT,
                rgbColors: COLORREF(0), // Only if CF_INITTOLOGFONTSTRUCT is set
                nFontType: REGULAR_FONTTYPE,
                ..Default::default()
            };

            if ChooseFontW(&mut CF).into() {
                Ok(DialogAction::Font {
                    name: cwide_to_string(&font.lfFaceName),
                    weight: FontWeight::from(font.lfWeight),
                    italic: font.lfItalic != 0,
                    underline: font.lfUnderline != 0,
                    strikethrough: font.lfStrikeOut != 0,
                    size: CF.iPointSize as u32 / 10
                })
            } else {
                get_dlg_error()
            }
        }
    }
}
