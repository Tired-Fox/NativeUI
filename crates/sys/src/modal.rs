use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::error::Error;

#[derive(Default, Debug, Clone, Copy)]
pub enum Buttons {
    #[default]
    Ok,
    OkCancel,
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

pub struct Dialog;
impl Dialog {
    pub fn prompt() -> Prompt {
        Prompt::default()
    }

    pub fn file() -> FileDialog {
        FileDialog::default()
    }
}

/// [Win32 Reference](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions)
#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
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

#[derive(Default, Debug, Clone)]
pub struct SaveFile {
    pub title: &'static str,
    pub filters: Option<Vec<(&'static str, &'static str)>>,
    pub directory: Option<&'static str>,
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
        self
    }

    pub fn directory(mut self, directory: &'static str) -> Self {
        self.directory = Some(directory);
        self
    }

    pub fn filters(mut self, filters: Vec<(&'static str, &'static str)>) -> Self {
        self.filters = Some(filters);
        self
    }
}

pub trait ToPath {
    fn to_path(&self) -> PathBuf;
}

#[derive(Debug, Clone)]
pub enum FileDialogAction {
    File(PathBuf),
    Files(Vec<PathBuf>),
    Canceled,
}

#[derive(Debug, Clone)]
pub struct FileDialog {
    pub title: Option<&'static str>,
    pub filters: Vec<(&'static str, Vec<&'static str>)>,
    pub filter_index: u32,
    pub filename: Option<&'static str>,
    pub default_extension: Option<&'static str>,
    pub default_folder: Option<&'static str>,
    pub folder: Option<&'static str>,
    pub options: HashSet<FileDialogOption>,
}

impl Default for FileDialog {
    fn default() -> Self {
        Self {
            title: None,
            filters: Vec::new(),
            filter_index: 1,
            filename: None,
            default_extension: None,
            default_folder: None,
            folder: None,
            options: HashSet::new(),
        }
    }
}

impl FileDialog {
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
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
        self
    }

    pub fn folder(mut self, directory: &'static str) -> Self {
        self.folder = Some(directory);
        self
    }

    pub fn filename(mut self, filename: &'static str) -> Self {
        self.filename = Some(filename);
        self
    }

    pub fn default_extension(mut self, extension: &'static str) -> Self {
        self.default_extension = Some(extension);
        self
    }

    pub fn default_folder(mut self, directory: &'static str) -> Self {
        self.default_folder = Some(directory);
        self
    }

    pub fn filter<const SIZE: usize>(
        mut self,
        name: &'static str,
        extensions: [&'static str; SIZE],
    ) -> Self {
        self.filters.push((name, Vec::from(extensions)));
        self
    }

    pub fn filter_index(mut self, filter_index: u32) -> Self {
        self.filter_index = filter_index;
        self
    }

    pub fn open(&self) -> Result<FileDialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::FileOpenDialog::new(&self)?.open()
    }

    pub fn save(&self) -> Result<FileDialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::FileSaveDialog::new(&self)?.save()
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

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn message(mut self, message: &'static str) -> Self {
        self.message = message;
        self
    }

    pub fn run(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            crate::windows::modal::MsgBox::new(self).show().unwrap()
        }
        // TODO: Linux and MacOS support
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }
}
