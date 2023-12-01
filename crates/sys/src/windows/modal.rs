use std::cell::RefCell;
use std::cmp::{max, min};
use std::path::PathBuf;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};

use windows::core::{w, GUID, HSTRING};
use windows::Win32::Foundation::{COLORREF, ERROR_CANCELLED, HWND, LPARAM};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitialize, CoUninitialize, CLSCTX_ALL, CLSCTX_INPROC_SERVER,
};
use windows::Win32::System::Diagnostics::Debug::FACILITY_WIN32;
use windows::Win32::UI::Controls::Dialogs::{
    ChooseColorW, CommDlgExtendedError, CC_ANYCOLOR, CC_FULLOPEN, CC_PREVENTFULLOPEN, CC_RGBINIT,
    CDERR_GENERALCODES, CHOOSECOLORW, CHOOSECOLOR_FLAGS, COMMON_DLG_ERRORS,
};
use windows::Win32::UI::Shell::Common::COMDLG_FILTERSPEC;
use windows::Win32::UI::Shell::{
    IFileOpenDialog, IFileSaveDialog, IShellItem, FILEOPENDIALOGOPTIONS, FOS_ALLOWMULTISELECT,
    FOS_CREATEPROMPT, FOS_FORCEPREVIEWPANEON, FOS_FORCESHOWHIDDEN, FOS_HIDEPINNEDPLACES,
    FOS_NOCHANGEDIR, FOS_NODEREFERENCELINKS, FOS_NOREADONLYRETURN, FOS_NOTESTFILECREATE,
    FOS_NOVALIDATE, FOS_OKBUTTONNEEDSINTERACTION, FOS_PICKFOLDERS, FOS_STRICTFILETYPES,
    SIGDN_DESKTOPABSOLUTEEDITING,
};
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDCANCEL, IDOK, MB_DEFAULT_DESKTOP_ONLY, MB_ICONEXCLAMATION, MB_ICONINFORMATION,
    MB_ICONQUESTION, MB_ICONWARNING, MB_OKCANCEL, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

use crate::error::Error;
use crate::modal::{
    Button, Buttons, DialogAction, FileDialog, FileDialogOption, Icon, Prompt, ToPath,
};
use crate::windows::{swap_rb, IntoPCWSTR};

// [Use Common Dialog Boxes](https://learn.microsoft.com/en-us/windows/win32/dlgbox/using-common-dialog-boxes)
thread_local! {
    static CUSTOM_COLORS: RefCell<Vec<COLORREF>> = RefCell::new(Vec::new());
}

macro_rules! e {
    ($e: expr) => {
        $e.map_err(|e| Into::<crate::error::Error>::into(e))
    };
}

macro_rules! shell_item {
    ($path: expr) => {
        // TODO: Convert error messages into more readable ones
        {
            let result: Result<::windows::Win32::UI::Shell::IShellItem, $crate::error::Error> =
                e!(::windows::Win32::UI::Shell::SHCreateItemFromParsingName(
                    ::windows::core::HSTRING::from(
                        &std::path::PathBuf::from($path)
                            .canonicalize()
                            .unwrap()
                            .display()
                            .to_string()[4..],
                    )
                    .as_pcwstr(),
                    None,
                ));
            result
        }
    };
}

macro_rules! hresult_from_win {
    ($win: expr) => {{
        let win = $win.0 as i32;
        if win <= 0 {
            ::windows::core::HRESULT(win)
        } else {
            ::windows::core::HRESULT(
                (($win.0 & 0x0000_FFFF) | (FACILITY_WIN32.0 << 16) | 0x8000_0000) as i32,
            )
        }
    }};
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

pub struct MsgBox<'a>(&'a Prompt);
impl<'a> MsgBox<'a> {
    pub fn new(context: &'a Prompt) -> Self {
        Self(context)
    }

    pub fn show(&self) -> Result<Button, Error> {
        Ok(unsafe {
            MessageBoxW(
                None,
                HSTRING::from(self.0.message).as_pcwstr(),
                HSTRING::from(self.0.title).as_pcwstr(),
                MB_DEFAULT_DESKTOP_ONLY
                    | MESSAGEBOX_STYLE::from(self.0.icon)
                    | MESSAGEBOX_STYLE::from(self.0.buttons),
            )
            .into()
        })
    }
}

enum IFileDialog {
    Open(IFileOpenDialog),
    Save(IFileSaveDialog),
}

impl IFileDialog {
    fn open() -> Result<Self, Error> {
        Ok(IFileDialog::Open(unsafe {
            CoCreateInstance(&CLSID_FILEOPENDIALOG, None, CLSCTX_INPROC_SERVER)?
        }))
    }

    fn save() -> Result<Self, Error> {
        Ok(IFileDialog::Save(unsafe {
            CoCreateInstance(&CLSID_FILESAVEDIALOG, None, CLSCTX_INPROC_SERVER)?
        }))
    }

    fn set_options(
        &self,
        add: Option<FILEOPENDIALOGOPTIONS>,
        remove: Option<FILEOPENDIALOGOPTIONS>,
    ) -> Result<(), Error> {
        let (get, set): (Box<dyn Fn() -> windows::core::Result<FILEOPENDIALOGOPTIONS>>, Box<dyn Fn(FILEOPENDIALOGOPTIONS) -> windows::core::Result<()>>)
            = match self {
                IFileDialog::Open(dialog) => (
                    Box::new(|| unsafe { dialog.GetOptions() }),
                    Box::new(|options: FILEOPENDIALOGOPTIONS| unsafe {
                        dialog.SetOptions(options)
                    }),
                ),
                IFileDialog::Save(dialog) => (
                    Box::new(|| unsafe { dialog.GetOptions() }),
                    Box::new(|options: FILEOPENDIALOGOPTIONS| unsafe {
                        dialog.SetOptions(options)
                    }),
                ),
            };

        let mut options = get()?;
        if let Some(add) = add {
            options |= add
        }
        if let Some(remove) = remove {
            options &= !remove
        }
        set(options)?;
        Ok(())
    }

    fn set_file_types(&self, filters: &[(HSTRING, HSTRING)]) -> Result<(), Error> {
        let filters: Vec<COMDLG_FILTERSPEC> = filters
            .iter()
            .map(|(k, v)| COMDLG_FILTERSPEC {
                pszName: k.as_pcwstr(),
                pszSpec: v.as_pcwstr(),
            })
            .collect();

        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    e!(dialog.SetFileTypes(&filters.as_slice()))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    e!(dialog.SetFileTypes(&filters.as_slice()))
                }
            },
        }
    }

    fn set_title(&self, title: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    e!(dialog.SetTitle(HSTRING::from(title).as_pcwstr()))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    e!(dialog.SetTitle(HSTRING::from(title).as_pcwstr()))
                }
            },
        }
    }

    fn set_folder(&self, path: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    e!(dialog.SetFolder(&shell_item!(path)?))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    e!(dialog.SetFolder(&shell_item!(path)?))
                }
            },
        }
    }

    fn set_default_folder(&self, path: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    e!(dialog.SetDefaultFolder(&shell_item!(path)?))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    e!(dialog.SetDefaultFolder(&shell_item!(path)?))
                }
            },
        }
    }

    fn set_filename(&self, name: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    e!(dialog.SetFileName(HSTRING::from(name).as_pcwstr()))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    e!(dialog.SetFileName(HSTRING::from(name).as_pcwstr()))
                }
            },
        }
    }

    fn set_default_extension(&self, extension: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    e!(dialog.SetDefaultExtension(HSTRING::from(extension).as_pcwstr()))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    e!(dialog.SetDefaultExtension(HSTRING::from(extension).as_pcwstr()))
                }
            },
        }
    }

    fn set_file_type_index(&self, index: u32) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    e!(dialog.SetFileTypeIndex(index))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    e!(dialog.SetFileTypeIndex(index))
                }
            },
        }
    }

    fn get_result(&self) -> Result<PathBuf, Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    Ok(e!(dialog.GetResult())?.to_path())
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    Ok(e!(dialog.GetResult())?.to_path())
                }
            },
        }
    }

    fn get_results(&self) -> Result<DialogAction, Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    let shell_items = e!(dialog.GetResults())?;
                    let capacity = unsafe { e!(shell_items.GetCount())? };
                    let mut paths = Vec::with_capacity(capacity as usize);
                    for i in 0..capacity {
                        paths.push(unsafe { e!(shell_items.GetItemAt(i))?.to_path() });
                    }
                    Ok(DialogAction::Files(paths))
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    Ok(DialogAction::File(e!(dialog.GetResult())?.to_path()))
                }
            },
        }
    }

    fn show(&self, parent: isize) -> Result<DialogAction, Error> {
        match self {
            IFileDialog::Open(dialog) => {
                unsafe {
                    match dialog.Show(HWND(parent)) {
                        Ok(_) => self.get_results(),
                        Err(e) => {
                            if e.code() == hresult_from_win!(ERROR_CANCELLED) {
                                Ok(DialogAction::Canceled)
                            } else {
                                Err(Error::from(e))
                            }
                        },
                    }
                }
            },
            IFileDialog::Save(dialog) => {
                unsafe {
                    match dialog.Show(HWND(parent)) {
                        Ok(_) => self.get_results(),
                        Err(e) => {
                            if e.code() == hresult_from_win!(ERROR_CANCELLED) {
                                Ok(DialogAction::Canceled)
                            } else {
                                Err(Error::from(e))
                            }
                        },
                    }
                }
            }
        }
    }
}

pub struct CommonFileDialog<'a> {
    context: &'a FileDialog,
    filters: Vec<(HSTRING, HSTRING)>,
}

impl From<&FileDialog> for FILEOPENDIALOGOPTIONS {
    fn from(v: &FileDialog) -> Self {
        v.options.iter().fold(FILEOPENDIALOGOPTIONS(0), |acc, v| {
            acc | (*v).into()
        })
    }
}

/// [Win32 Example](https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/winui/shell/appplatform/commonfiledialog/CommonFileDialogApp.cpp)
impl<'a> CommonFileDialog<'a> {
    pub fn new(context: &'a FileDialog) -> Self {
        Self {
            context,
            // Filter strings need to have a static memory location while the dialog is open
            // so create the HSTRINGS here so they live longer than the modal.
            filters: context
                .filters
                .iter()
                .map(|(k, v)| {
                    (
                        HSTRING::from(*k),
                        HSTRING::from(
                            v.iter()
                                .map(|v| format!("*.{}", v))
                                .collect::<Vec<String>>()
                                .join(";"),
                        ),
                    )
                })
                .collect(),
        }
    }

    fn set_start_directory(&self, dialog: &IFileDialog) -> Result<(), Error> {
        if let Some(folder) = self.context.directory {
            dialog.set_folder(folder)?;
        } else if let Some(folder) = self.context.default_folder {
            dialog.set_default_folder(folder)?;
        }
        Ok(())
    }

    fn set_filename_extension(&self, dialog: &IFileDialog, filename: Option<&str>, extension: Option<&str>) -> Result<(), Error> {
        if let Some(filename) = filename {
            dialog.set_filename(filename)?;
        }
        if let Some(extension) = extension {
            dialog.set_default_extension(extension)?;
        }
        Ok(())
    }

    pub fn pick_file(&self, parent: isize) -> Result<DialogAction, Error> {
        let dialog = IFileDialog::open()?;
        dialog.set_options(Some(self.context.into()), None)?;
        dialog.set_file_types(self.filters.as_slice())?;
        if let Some(title) = self.context.title {
            dialog.set_title(title)?;
        }

        self.set_start_directory(&dialog)?;
        dialog.set_file_type_index(self.context.filter_index)?;

        dialog.show(parent)
    }

    pub fn save_file(&self, parent: isize) -> Result<DialogAction, Error> {
        let dialog = IFileDialog::save()?;
        dialog.set_options(Some(self.context.into()), Some(FOS_ALLOWMULTISELECT))?;
        dialog.set_file_types(self.filters.as_slice())?;
        if let Some(title) = self.context.title {
            dialog.set_title(title)?;
        }

        self.set_filename_extension(&dialog, self.context.filename, self.context.default_extension)?;
        self.set_start_directory(&dialog)?;
        dialog.set_file_type_index(self.context.filter_index)?;

        dialog.show(parent)
    }

    pub fn pick_folder(&self, parent: isize) -> Result<DialogAction, Error> {
        let dialog = IFileDialog::open()?;
        dialog.set_options(Some(FILEOPENDIALOGOPTIONS::from(self.context) | FOS_PICKFOLDERS), None)?;
        if let Some(title) = self.context.title {
            dialog.set_title(title)?;
        }

        self.set_start_directory(&dialog)?;
        dialog.show(parent)
    }
}

#[derive(Default, Debug, Clone)]
pub struct ColorPicker {
    expanded: bool,
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
                    let error = unsafe { CommDlgExtendedError() };
                    if error == CDERR_GENERALCODES {
                        Ok(DialogAction::Canceled)
                    } else {
                        Err(Error::from(error))
                    }
                }
            }
        })
    }

    pub fn show(&self) -> Result<DialogAction, Error> {
        self.show_with(0)
    }
}
