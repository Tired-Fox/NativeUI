use std::ffi::c_void;
use std::fmt::Debug;
use std::sync::Arc;

use windows::core::HSTRING;
use windows::Foundation::{EventRegistrationToken, TypedEventHandler};
use windows::Win32::Foundation::{BOOL, HANDLE, HMODULE, HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};
use windows::Win32::Graphics::Gdi::{GetDC, UpdateWindow};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, CloseWindow, CreateWindowExW, LoadCursorW, LoadImageW, RegisterClassW,
    ShowWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, HICON, IDC_ARROW, IMAGE_ICON,
    LR_DEFAULTSIZE, LR_LOADFROMFILE, LR_LOADTRANSPARENT, LR_SHARED, SW_HIDE, SW_MAXIMIZE,
    SW_MINIMIZE, SW_RESTORE, SW_SHOWNORMAL, WINDOW_EX_STYLE, WM_ERASEBKGND, WM_PAINT, WNDCLASSW,
    WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};
use windows::UI::ViewManagement::UISettings;

use crate::windows::win_error::WinError;

use super::{event::wnd_proc, is_dark_mode, IntoPCWSTR, UI_SETTINGS, Background};

enum ColorMode {
    Light = 0,
    Dark = 20,
}

impl From<ColorMode> for DWMWINDOWATTRIBUTE {
    fn from(color_mode: ColorMode) -> Self {
        DWMWINDOWATTRIBUTE(color_mode as i32)
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    Auto,
}

pub type Handler = Arc<dyn Fn(HWND, u32, WPARAM, LPARAM) -> bool + Send + Sync + 'static>;

#[derive(Default)]
pub struct Builder {
    options: WindowOptions,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            options: WindowOptions::default(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.options.title = HSTRING::from(title);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.options.theme = theme;
        self
    }

    pub fn background(mut self, background: Background) -> Self {
        self.options.background = background;
        self
    }

    pub fn show(mut self) -> Self {
        self.options.show = true;
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.options.icon = Some(HSTRING::from(icon));
        self
    }

    pub fn create(self) -> windows::core::Result<Box<Window>> {
        Window::create(self.options)
    }
}

pub struct WindowOptions {
    pub title: HSTRING,
    pub class: HSTRING,
    pub icon: Option<HSTRING>,

    pub theme: Theme,
    pub background: Background,

    pub show: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: HSTRING::from(""),
            class: HSTRING::from(format!("Window-Cypress-{}", uuid::Uuid::new_v4())),
            icon: None,

            theme: Theme::Auto,
            background: Background::default(),

            show: false,
        }
    }
}

impl Debug for WindowOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowOptions")
            .field("title", &self.title)
            .field("class", &self.class)
            .field("theme", &self.theme)
            .finish()
    }
}

pub struct Window {
    handle: HWND,
    instance: HMODULE,
    options: WindowOptions,

    theme_cookie: Option<EventRegistrationToken>,
}

impl Window {
    pub fn create(options: WindowOptions) -> windows::core::Result<Box<Self>> {
        let mut window = Box::new(Window {
            handle: HWND(0),
            instance: HMODULE(0),
            options,
            theme_cookie: None,
        });

        unsafe {
            window.instance = GetModuleHandleW(None)?;
            debug_assert!(window.instance.0 != 0);

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: window.instance.into(),
                lpszClassName: window.class().as_pcwstr(),
                hIcon: icon(window.options.icon.as_ref()),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wnd_proc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

            window.handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window.class().as_pcwstr(),
                window.title().as_pcwstr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                window.instance.clone(),
                Some(&window.options as *const _ as *const _),
            );
        };

        window.set_theme(window.options.theme)?;
        if window.options.show {
            window.show();
        }
        Ok(window)
    }

    pub fn set_theme(&mut self, theme: Theme) -> ::windows::core::Result<()> {
        let state = match theme {
            Theme::Light => {
                if let Some(cookie) = self.theme_cookie {
                    UI_SETTINGS.RemoveColorValuesChanged(cookie)?;
                }
                BOOL(0)
            }
            Theme::Dark => {
                if let Some(cookie) = self.theme_cookie {
                    UI_SETTINGS.RemoveColorValuesChanged(cookie)?;
                }
                BOOL(1)
            }
            Theme::Auto => unsafe {
                let handle = self.handle;
                self.theme_cookie = Some(UI_SETTINGS.ColorValuesChanged(
                    &TypedEventHandler::new(move |settings: &Option<UISettings>, _| {
                        if settings.is_some() {
                            DwmSetWindowAttribute(
                                handle,
                                ColorMode::Dark.into(),
                                &is_dark_mode() as *const _ as *const _,
                                4,
                            )
                            .unwrap();
                            CallWindowProcW(
                                Some(wnd_proc),
                                handle,
                                WM_ERASEBKGND,
                                WPARAM(GetDC(handle).0 as usize),
                                LPARAM(0),
                            );
                            CallWindowProcW(Some(wnd_proc), handle, WM_PAINT, WPARAM(0), LPARAM(0));
                        }
                        Ok(())
                    }),
                )?);

                is_dark_mode()
            },
        };

        unsafe {
            DwmSetWindowAttribute(
                self.handle(),
                ColorMode::Dark.into(),
                &state as *const _ as *const c_void,
                4,
            )?;
        }

        Ok(())
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Show the window
    pub fn show(&self) {
        unsafe {
            ShowWindow(self.handle(), SW_SHOWNORMAL);
        }
    }

    /// Hide the window
    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.handle(), SW_HIDE);
        }
    }

    /// Minimize the window
    pub fn minimize(&self) {
        unsafe {
            ShowWindow(self.handle(), SW_MINIMIZE);
        }
    }

    /// Restore the window
    pub fn restore(&self) {
        unsafe {
            ShowWindow(self.handle(), SW_RESTORE);
        }
    }

    /// Maximize the window
    pub fn maximize(&self) {
        unsafe {
            ShowWindow(self.handle(), SW_MAXIMIZE);
        }
    }

    pub fn update(&self) {
        unsafe {
            UpdateWindow(self.handle());
        }
    }

    pub fn close(&self) -> windows::core::Result<()> {
        unsafe { CloseWindow(self.handle()) }
    }

    pub fn handle(&self) -> HWND {
        HWND(self.handle.0)
    }

    pub fn class(&self) -> HSTRING {
        self.options.class.clone()
    }

    pub fn title(&self) -> HSTRING {
        self.options.title.clone()
    }
}

/// TODO: Automatic loading of other file formats?
pub fn icon(path: Option<&HSTRING>) -> HICON {
    let result = HICON(path.map_or(0, |icon| unsafe {
        match LoadImageW(
            None,
            icon.as_pcwstr(),
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE | LR_LOADFROMFILE | LR_SHARED | LR_LOADTRANSPARENT,
        ) {
            Ok(hicon) => hicon,
            Err(err) => {
                println!("{}", WinError::from(err));
                HANDLE(0)
            }
        }
        .0
    }));
    result
}
