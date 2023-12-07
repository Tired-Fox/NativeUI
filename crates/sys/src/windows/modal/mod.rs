use std::path::PathBuf;

use windows::core::{GUID, HRESULT, HSTRING};
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Diagnostics::Debug::FACILITY_WIN32;
use windows::Win32::UI::Controls::Dialogs::{CommDlgExtendedError, CDERR_GENERALCODES};
use windows::Win32::UI::Shell::{
    IShellItem, SHCreateItemFromParsingName, FILEOPENDIALOGOPTIONS, FOS_ALLOWMULTISELECT,
    FOS_CREATEPROMPT, FOS_FORCESHOWHIDDEN, FOS_HIDEPINNEDPLACES, FOS_NOCHANGEDIR,
    FOS_NODEREFERENCELINKS, FOS_NOREADONLYRETURN, FOS_NOTESTFILECREATE, FOS_NOVALIDATE,
    FOS_OKBUTTONNEEDSINTERACTION, FOS_PICKFOLDERS, FOS_STRICTFILETYPES,
    SIGDN_DESKTOPABSOLUTEEDITING,
};
use windows::Win32::UI::WindowsAndMessaging::{
    IDCANCEL, MB_ICONEXCLAMATION, MB_ICONINFORMATION, MB_ICONQUESTION, MB_ICONWARNING, MB_OKCANCEL,
    MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

pub use color::ColorPicker;
pub use file::CommonFileDialog;
pub use font::FontDialog;
pub use prompt::MsgBox;

use crate::e;
use crate::error::Error;
use crate::modal::{Button, Buttons, DialogAction, FileDialogOption, Icon, ToPath};
use crate::windows::IntoPCWSTR;

mod color;
mod file;
mod font;
mod prompt;

// [Use Common Dialog Boxes](https://learn.microsoft.com/en-us/windows/win32/dlgbox/using-common-dialog-boxes)

pub(crate) fn to_shell_item(path: &str) -> Result<IShellItem, Error> {
    //TODO: Auto expand `~`
    let path = PathBuf::from(path);

    println!("{:?}", path);
    let path = match path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            return Err(Error {
                code: 3,
                message: format!("File not found at path: {:?}", path.display().to_string()),
            })
        }
    };

    let result: Result<IShellItem, Error> = unsafe {
        e!(SHCreateItemFromParsingName(
            HSTRING::from(&path.canonicalize().unwrap().display().to_string()[4..],).as_pcwstr(),
            None,
        ))
    };
    result
}

pub(crate) fn hresult_from_win(err: WIN32_ERROR) -> HRESULT {
    let win = err.0 as i32;
    if win <= 0 {
        ::windows::core::HRESULT(win)
    } else {
        ::windows::core::HRESULT(
            ((err.0 & 0x0000_FFFF) | (FACILITY_WIN32.0 << 16) | 0x8000_0000) as i32,
        )
    }
}

macro_rules! DEFINE_GUID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr
    ) => {
        pub const $name: GUID = GUID {
            data1: $l,
            data2: $w1,
            data3: $w2,
            data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    };
}

DEFINE_GUID! {
    CLSID_FILEOPENDIALOG,
    0xdc1c5a9c, 0xe88a, 0x4dde, 0xa5, 0xa1, 0x60, 0xf8, 0x2a, 0x20, 0xae, 0xf7
}
DEFINE_GUID! {
    CLSID_FILESAVEDIALOG,
    0xc0b4e2f3, 0xba21, 0x4773, 0x8d, 0xba, 0x33, 0x5e, 0xc9, 0x46, 0xeb, 0x8b
}

fn get_dlg_error() -> Result<DialogAction, Error> {
    let error = unsafe { CommDlgExtendedError() };
    if error == CDERR_GENERALCODES {
        Ok(DialogAction::Canceled)
    } else {
        Err(Error::from(error))
    }
}

impl ToPath for IShellItem {
    fn to_path(&self) -> PathBuf {
        PathBuf::from(unsafe {
            self.GetDisplayName(SIGDN_DESKTOPABSOLUTEEDITING)
                .unwrap()
                .to_hstring()
                .unwrap()
                .to_string_lossy()
        })
    }
}

impl From<FileDialogOption> for FILEOPENDIALOGOPTIONS {
    fn from(v: FileDialogOption) -> Self {
        match v {
            FileDialogOption::StrictFileTypes => FOS_STRICTFILETYPES,
            FileDialogOption::NoChangeDir => FOS_NOCHANGEDIR,
            FileDialogOption::PickFolders => FOS_PICKFOLDERS,
            FileDialogOption::NoValidate => FOS_NOVALIDATE,
            FileDialogOption::AllowMultiSelect => FOS_ALLOWMULTISELECT,
            FileDialogOption::CreatePrompt => FOS_CREATEPROMPT,
            FileDialogOption::NoReadOnlyReturn => FOS_NOREADONLYRETURN,
            FileDialogOption::NoTestFileCreate => FOS_NOTESTFILECREATE,
            FileDialogOption::HidePinnedPlaces => FOS_HIDEPINNEDPLACES,
            FileDialogOption::NoDereferenceLinks => FOS_NODEREFERENCELINKS,
            FileDialogOption::OkButtonNeedsInteraction => FOS_OKBUTTONNEEDSINTERACTION,
            FileDialogOption::ForceShowHidden => FOS_FORCESHOWHIDDEN,
        }
    }
}

impl From<Icon> for MESSAGEBOX_STYLE {
    fn from(v: Icon) -> Self {
        match v {
            Icon::Exclamation => MB_ICONEXCLAMATION,
            Icon::Information => MB_ICONINFORMATION,
            Icon::Question => MB_ICONQUESTION,
            Icon::Warning => MB_ICONWARNING,
            Icon::None => MESSAGEBOX_STYLE(0),
        }
    }
}

impl From<Buttons> for MESSAGEBOX_STYLE {
    fn from(v: Buttons) -> Self {
        match v {
            Buttons::OkCancel => MB_OKCANCEL,
            Buttons::Ok => MB_OKCANCEL,
        }
    }
}

impl From<MESSAGEBOX_RESULT> for Button {
    fn from(v: MESSAGEBOX_RESULT) -> Self {
        match v {
            IDCANCEL => Button::Cancel,
            _ => Button::Ok,
        }
    }
}

/// Convert a wide null terminated cstring to a String
pub(crate) fn cwide_to_string(src: &[u16]) -> String {
    let mut end = 0usize;
    for b in src {
        if *b == 0 {
            break;
        }
        end += 1;
    }
    String::from_utf16_lossy(&src[0..end])
}

/*
* * NOTE: More Modals:
* * - Print Modal
* * - Find and Replace Modal
*/
