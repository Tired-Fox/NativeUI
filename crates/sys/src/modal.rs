use std::collections::HashSet;
use std::path::PathBuf;

use crate::error::Error;

// TODO: Add more button combinations
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Buttons {
    #[default]
    Ok,
    OkCancel,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Button {
    Ok,
    Cancel,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Icon {
    Exclamation,
    Information,
    Question,
    Warning,
    #[default]
    None,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FontWeight {
    #[default]
    Any = 0,
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

pub struct Dialog;
impl Dialog {
    pub fn prompt() -> Prompt {
        Prompt::default()
    }
    pub fn file() -> FileDialog {
        FileDialog::default()
    }
    pub fn color() -> ColorDialog {
        ColorDialog::default()
    }
    pub fn font() -> Font {
        Font::default()
    }
}

#[derive(Default, Debug, Clone)]
pub struct ColorDialog {
    initial_color: Option<u32>,
}

impl ColorDialog {
    /// Set the initial color on dialog open
    pub fn initial(mut self, initial_color: u32) -> Self {
        self.initial_color = Some(initial_color);
        self
    }

    /// Show the color dialog
    pub fn show(&self) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::ColorPicker::new(self.initial_color).show()
    }

    /// Get the current custom colors set by the user
    pub fn get_custom_colors() -> Vec<u32> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::ColorPicker::get_custom_colors()
    }

    /// Set the current custom colors for the dialog
    pub fn set_custom_colors(colors: Vec<u32>) {
        #[cfg(target_os = "windows")]
        crate::windows::modal::ColorPicker::set_custom_colors(colors)
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

pub trait ToPath {
    fn to_path(&self) -> PathBuf;
}

#[derive(Debug, Clone)]
pub enum DialogAction {
    Files(Vec<PathBuf>),
    File(PathBuf),
    Color(u32),
    Font {
        name: String,
        size: u32,
        italic: bool,
        underline: bool,
        strikethrough: bool,
        weight: FontWeight,
    },
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
    pub directory: Option<&'static str>,
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
            directory: None,
            options: HashSet::new(),
        }
    }
}

impl FileDialog {
    /// Set the title of the dialog window
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }

    /// When the file dialog opens allow the user to select multiple options
    pub fn multiple(mut self) -> Self {
        self.options.insert(FileDialogOption::AllowMultiSelect);
        self
    }

    /// Show hidden files and folders
    pub fn show_hidden(mut self) -> Self {
        self.options.insert(FileDialogOption::ForceShowHidden);
        self
    }

    /// Set the starting directory the dialog opens in
    pub fn directory(mut self, directory: &'static str) -> Self {
        self.directory = Some(directory);
        self
    }

    /// Set the filename and the default extension for when the dialog opens
    pub fn filename(mut self, filename: &'static str, extension: &'static str) -> Self {
        self.filename = Some(filename);
        self.default_extension = Some(extension);
        self
    }

    /// Set the default starting directory, this will change as the user opens new dialogs
    pub fn default_folder(mut self, directory: &'static str) -> Self {
        self.default_folder = Some(directory);
        self
    }

    /// Set the file type filters.
    ///
    /// First is the index of the filter to use by default starting from one. Second is a list of filter name to extension patterns tuples.
    /// On windows this will combine the patterns into a single string. Ex: `("Images", ["*.png", "*.jpg"])` will become `Images (*.png;*.jpg)`
    pub fn filters(mut self, index: u32, filters: &[(&'static str, &[&'static str])]) -> Self {
        self.filters.extend(
            filters
                .iter()
                .map(|(name, extensions)| (*name, Vec::from(*extensions))),
        );
        self.filter_index = index;
        self
    }

    /// Take the current options and create an open file dialog
    pub fn open_file(&self) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::CommonFileDialog::new(&self).pick_file(0)
    }

    /// Take the current options and create a save file dialog
    pub fn save_file(&self) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::CommonFileDialog::new(&self).save_file(0)
    }

    /// Take the current options and create an open folder dialog
    pub fn open_folder(&self) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::CommonFileDialog::new(&self).pick_folder(0)
    }

    /// Take the current options and create an open file dialog
    pub fn open_file_with(&self, parent: isize) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::CommonFileDialog::new(&self).pick_file(parent)
    }

    /// Take the current options and create a save file dialog
    pub fn save_file_with(&self, parent: isize) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::CommonFileDialog::new(&self).save_file(parent)
    }

    /// Take the current options and create an open folder dialog
    pub fn open_folder_with(&self, parent: isize) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::CommonFileDialog::new(&self).pick_folder(parent)
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
    /// Set the button combination displayed on the dialog
    pub fn buttons(mut self, buttons: Buttons) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set the icon that is used on the dialog
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    /// Set the dialog title
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// Set the dialog message
    pub fn message(mut self, message: &'static str) -> Self {
        self.message = message;
        self
    }

    /// Show the dialog
    pub fn show(&self) -> Button {
        #[cfg(target_os = "windows")]
        {
            crate::windows::modal::MsgBox::new(self)
                .show()
                .unwrap_or(Button::Cancel)
        }
        // TODO: Linux and MacOS support
        #[cfg(not(target_os = "windows"))]
        {
            Button::Cancel
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Font {
    pub size: u32,
    pub weight: FontWeight,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

impl Font {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
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

    pub fn show(&self) -> Result<DialogAction, Error> {
        #[cfg(target_os = "windows")]
        crate::windows::modal::FontDialog {
            point_size: self.size * 10,
            weight: self.weight,
            italic: self.italic,
            underline: self.underline,
            strikethrough: self.strikethrough,
        }
        .show()
    }
}
