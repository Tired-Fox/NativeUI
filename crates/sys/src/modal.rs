use std::collections::HashSet;
use windows::Win32::System::Com::CoCreateInstance;

#[cfg(target_os = "windows")]
use windows::Win32::UI::{
    Shell::{FILEOPENDIALOGOPTIONS, FOS_FILEMUSTEXIST},
    WindowsAndMessaging::{
        IDOK, MB_DEFAULT_DESKTOP_ONLY, MB_ICONEXCLAMATION, MB_ICONINFORMATION, MB_ICONQUESTION,
        MB_ICONWARNING, MB_OKCANCEL, MESSAGEBOX_STYLE,
    },
};
#[cfg(target_os = "windows")]
use crate::windows::IntoPCWSTR;

#[derive(Default, Debug, Clone, Copy)]
pub enum Buttons {
    #[default]
    Ok,
    OkCancel,
}

impl From<Buttons> for MESSAGEBOX_STYLE {
    fn from(v: Buttons) -> Self {
        match v {
            Buttons::OkCancel => MB_OKCANCEL,
            Buttons::Ok => MB_OKCANCEL,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Icon {
    Exclamation,
    Information,
    Question,
    Warning,
    #[default]
    None,
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

pub struct Dialog;
impl Dialog {
    pub fn prompt() -> Prompt {
        Prompt::default()
    }

    pub fn open_file() -> OpenFile {
        OpenFile::default()
    }

    pub fn save_file() -> SaveFile {
        SaveFile::default()
    }
}

/// [Win32 Reference](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions)
#[derive(Default, Debug, Clone, Copy, Hash)]
pub enum FileDialogOption {
    /// User can only choose a file with the extension provided through `file_types`
    StrictFileTypes,
    /// Pick folders only
    PickFolders,
    /// Allow multiple selections
    AllowMultiSelect,
    /// Force hidden and system files to be shown
    ForceShowHidden,
    /// Hide the pinned locations in the file selector
    HidePinnedPlaces,
    NoChangeDir,
    NoValidate,
    CreatePrompt,
    NoReadOnlyReturn,
    NoTestFileCreate,
    NoDereferenceLinks,
    OkButtonNeedsInteraction,
}

impl From<FileDialogOption> for FILEOPENDIALOGOPTIONS {
    fn from(v: FileDialogOption) -> Self {
        match v {
            FileDialogOption::StrictFileTypes => FOS_FILEMUSTEXIST,
            FileDialogOption::NoChangeDir => FOS_FILEMUSTEXIST,
            FileDialogOption::PickFolders => FOS_FILEMUSTEXIST,
            FileDialogOption::NoValidate => FOS_FILEMUSTEXIST,
            FileDialogOption::AllowMultiSelect => FOS_FILEMUSTEXIST,
            FileDialogOption::CreatePrompt => FOS_FILEMUSTEXIST,
            FileDialogOption::NoReadOnlyReturn => FOS_FILEMUSTEXIST,
            FileDialogOption::NoTestFileCreate => FOS_FILEMUSTEXIST,
            FileDialogOption::HidePinnedPlaces => FOS_FILEMUSTEXIST,
            FileDialogOption::NoDereferenceLinks => FOS_FILEMUSTEXIST,
            FileDialogOption::OkButtonNeedsInteraction => FOS_FILEMUSTEXIST,
            FileDialogOption::ForceShowHidden => FOS_FILEMUSTEXIST,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct SaveFile {
    pub title: &'static str,
    pub options: HashSet<FileDialogOption>,
}

impl SaveFile {
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn build(self) -> Vec<String> {
        Vec::new()
    }

    pub fn strict_file_types(mut self) -> Self {
        self.options.insert(FileDialogOption::StrictFileTypes);
        self
    }

    pub fn pick_folders(mut self) -> Self {
        self.options.insert(FileDialogOption::PickFolders);
        self
    }

    pub fn show_hidden(mut self) -> Self {
        self.options.insert(FileDialogOption::ForceShowHidden);
    }
}

#[derive(Default, Debug, Clone)]
pub struct OpenFile {
    pub title: &'static str,
    pub options: HashSet<FileDialogOption>,
}

impl OpenFile {
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn pick_folders(mut self) -> Self {
        self.options.insert(FileDialogOption::PickFolders);
        self
    }

    pub fn multi_select(mut self) -> Self {
        self.options.insert(FileDialogOption::AllowMultiSelect);
        self
    }

    pub fn show_hidden(mut self) -> Self {
        self.options.insert(FileDialogOption::ForceShowHidden);
    }

    pub fn build(self) -> Vec<String> {
        Vec::new()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Prompt {
    pub title: &'static str,
    pub message: &'static str,
    pub buttons: Buttons,
    pub icon: Icon,
}

impl Prompt {
    pub fn buttons(mut self, buttons: Buttons) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message;
        self
    }

    pub fn build(self) -> bool {
        #[cfg(target_os = "windows")]
        unsafe {
            use crate::windows::IntoPCWSTR;
            use windows::core::HSTRING;
            use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;

            MessageBoxW(
                None,
                HSTRING::from(self.message).as_pcwstr(),
                HSTRING::from(self.title).as_pcwstr(),
                MB_DEFAULT_DESKTOP_ONLY
                    | MESSAGEBOX_STYLE::from(self.icon)
                    | MESSAGEBOX_STYLE::from(self.buttons),
            ) == IDOK
        }
        // TODO: Linux and MacOS support
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }
}
