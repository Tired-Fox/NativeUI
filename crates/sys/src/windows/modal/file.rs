use windows::core::HSTRING;
use windows::Win32::Foundation::{ERROR_CANCELLED, HWND};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::UI::Shell::Common::COMDLG_FILTERSPEC;
use windows::Win32::UI::Shell::{
    IFileOpenDialog, IFileSaveDialog, FILEOPENDIALOGOPTIONS, FOS_ALLOWMULTISELECT, FOS_PICKFOLDERS,
};

use crate::e;
use crate::error::Error;
use crate::modal::{DialogAction, FileDialog, ToPath};
use crate::windows::IntoPCWSTR;

use super::{hresult_from_win, to_shell_item, CLSID_FILEOPENDIALOG, CLSID_FILESAVEDIALOG};

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
        let (get, set): (
            Box<dyn Fn() -> windows::core::Result<FILEOPENDIALOGOPTIONS>>,
            Box<dyn Fn(FILEOPENDIALOGOPTIONS) -> windows::core::Result<()>>,
        ) = match self {
            IFileDialog::Open(dialog) => (
                Box::new(|| unsafe { dialog.GetOptions() }),
                Box::new(|options: FILEOPENDIALOGOPTIONS| unsafe { dialog.SetOptions(options) }),
            ),
            IFileDialog::Save(dialog) => (
                Box::new(|| unsafe { dialog.GetOptions() }),
                Box::new(|options: FILEOPENDIALOGOPTIONS| unsafe { dialog.SetOptions(options) }),
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
            IFileDialog::Open(dialog) => unsafe { e!(dialog.SetFileTypes(&filters.as_slice())) },
            IFileDialog::Save(dialog) => unsafe { e!(dialog.SetFileTypes(&filters.as_slice())) },
        }
    }

    fn set_title(&self, title: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => unsafe {
                e!(dialog.SetTitle(HSTRING::from(title).as_pcwstr()))
            },
            IFileDialog::Save(dialog) => unsafe {
                e!(dialog.SetTitle(HSTRING::from(title).as_pcwstr()))
            },
        }
    }

    fn set_folder(&self, path: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => unsafe { e!(dialog.SetFolder(&to_shell_item(path)?)) },
            IFileDialog::Save(dialog) => unsafe { e!(dialog.SetFolder(&to_shell_item(path)?)) },
        }
    }

    fn set_default_folder(&self, path: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => unsafe {
                e!(dialog.SetDefaultFolder(&to_shell_item(path)?))
            },
            IFileDialog::Save(dialog) => unsafe {
                e!(dialog.SetDefaultFolder(&to_shell_item(path)?))
            },
        }
    }

    fn set_filename(&self, name: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => unsafe {
                e!(dialog.SetFileName(HSTRING::from(name).as_pcwstr()))
            },
            IFileDialog::Save(dialog) => unsafe {
                e!(dialog.SetFileName(HSTRING::from(name).as_pcwstr()))
            },
        }
    }

    fn set_default_extension(&self, extension: &str) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => unsafe {
                e!(dialog.SetDefaultExtension(HSTRING::from(extension).as_pcwstr()))
            },
            IFileDialog::Save(dialog) => unsafe {
                e!(dialog.SetDefaultExtension(HSTRING::from(extension).as_pcwstr()))
            },
        }
    }

    fn set_file_type_index(&self, index: u32) -> Result<(), Error> {
        match self {
            IFileDialog::Open(dialog) => unsafe { e!(dialog.SetFileTypeIndex(index)) },
            IFileDialog::Save(dialog) => unsafe { e!(dialog.SetFileTypeIndex(index)) },
        }
    }

    // fn get_result(&self) -> Result<PathBuf, Error> {
    //     match self {
    //         IFileDialog::Open(dialog) => unsafe { Ok(e!(dialog.GetResult())?.to_path()) },
    //         IFileDialog::Save(dialog) => unsafe { Ok(e!(dialog.GetResult())?.to_path()) },
    //     }
    // }

    fn get_results(&self) -> Result<DialogAction, Error> {
        match self {
            IFileDialog::Open(dialog) =>  {
                let shell_items = unsafe { e!(dialog.GetResults())? };
                let capacity = unsafe { e!(shell_items.GetCount())? };
                let mut paths = Vec::with_capacity(capacity as usize);
                for i in 0..capacity {
                    paths.push(unsafe { e!(shell_items.GetItemAt(i))?.to_path() });
                }
                Ok(DialogAction::Files(paths))
            },
            IFileDialog::Save(dialog) => unsafe {
                Ok(DialogAction::File(e!(dialog.GetResult())?.to_path()))
            },
        }
    }

    fn show(&self, parent: isize) -> Result<DialogAction, Error> {
        match self {
            IFileDialog::Open(dialog) => unsafe {
                match dialog.Show(HWND(parent)) {
                    Ok(_) => self.get_results(),
                    Err(e) => {
                        if e.code() == hresult_from_win(ERROR_CANCELLED) {
                            Ok(DialogAction::Canceled)
                        } else {
                            Err(Error::from(e))
                        }
                    }
                }
            },
            IFileDialog::Save(dialog) => unsafe {
                match dialog.Show(HWND(parent)) {
                    Ok(_) => self.get_results(),
                    Err(e) => {
                        if e.code() == hresult_from_win(ERROR_CANCELLED) {
                            Ok(DialogAction::Canceled)
                        } else {
                            Err(Error::from(e))
                        }
                    }
                }
            },
        }
    }
}

pub struct CommonFileDialog<'a> {
    context: &'a FileDialog,
    filters: Vec<(HSTRING, HSTRING)>,
}

impl From<&FileDialog> for FILEOPENDIALOGOPTIONS {
    fn from(v: &FileDialog) -> Self {
        v.options
            .iter()
            .fold(FILEOPENDIALOGOPTIONS(0), |acc, v| acc | (*v).into())
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

    fn set_filename_extension(
        &self,
        dialog: &IFileDialog,
        filename: Option<&str>,
        extension: Option<&str>,
    ) -> Result<(), Error> {
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

        self.set_filename_extension(
            &dialog,
            self.context.filename,
            self.context.default_extension,
        )?;
        self.set_start_directory(&dialog)?;
        dialog.set_file_type_index(self.context.filter_index)?;

        dialog.show(parent)
    }

    pub fn pick_folder(&self, parent: isize) -> Result<DialogAction, Error> {
        let dialog = IFileDialog::open()?;
        dialog.set_options(
            Some(FILEOPENDIALOGOPTIONS::from(self.context) | FOS_PICKFOLDERS),
            None,
        )?;
        if let Some(title) = self.context.title {
            dialog.set_title(title)?;
        }

        self.set_start_directory(&dialog)?;
        dialog.show(parent)
    }
}
