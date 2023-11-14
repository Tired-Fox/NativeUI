use windows::core::{HSTRING, PCSTR, PCWSTR};
use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};
use windows::UI::ViewManagement::UISettings;

pub mod event;
pub mod window;

lazy_static::lazy_static! {
    pub static ref UI_SETTINGS: UISettings = UISettings::new().unwrap();
}

pub trait IntoPCSTR {
    fn as_pcstr(&self) -> PCSTR;
}
pub trait IntoPCWSTR {
    fn as_pcwstr(&self) -> PCWSTR;
}

impl IntoPCSTR for HSTRING {
    fn as_pcstr(&self) -> PCSTR {
        PCSTR::from_raw(self.as_ptr() as _)
    }
}

impl IntoPCWSTR for HSTRING {
    fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR::from_raw(self.as_ptr() as _)
    }
}
pub fn is_light_theme() -> bool {
    // based on https://stackoverflow.com/a/51336913/709884
    let mut buffer: [u8; 4] = [0; 4];
    let mut cb_data: u32 = (buffer.len()).try_into().unwrap();
    let res = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            HSTRING::from(r#"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"#)
                .as_pcwstr(),
            HSTRING::from("AppsUseLightTheme").as_pcwstr(),
            RRF_RT_REG_DWORD,
            None,
            Some(buffer.as_mut_ptr() as _),
            Some(&mut cb_data as *mut _),
        )
    };

    if let Err(err) = res {
        panic!("failed to read key from registry: err_code={:?}", unsafe { err.info().as_ref().unwrap().GetReference().unwrap().as_wide() });
    }

    // REG_DWORD is signed 32-bit, using little endian
    let light_mode = i32::from_le_bytes(buffer) == 1;
    light_mode
}

pub fn is_dark_theme() -> bool {
    !is_light_theme()
}
