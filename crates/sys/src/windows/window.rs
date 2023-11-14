use std::ffi::c_void;
use std::fmt::Debug;
use std::sync::Arc;

use windows::core::HSTRING;
use windows::Foundation::{EventRegistrationToken, TypedEventHandler};
use windows::UI::Color;
use windows::UI::ViewManagement::{UIColorType, UISettings};
use windows::Win32::Foundation::{BOOL, HMODULE, HWND, LPARAM, WPARAM};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CloseWindow, CreateWindowExW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, LoadCursorW, RegisterClassW, ShowWindow, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOWNORMAL, WINDOW_EX_STYLE, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_SIZEBOX};

use super::{event::wnd_proc, IntoPCWSTR, UI_SETTINGS};

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

    pub fn show(mut self) -> Self {
        self.options.show = true;
        self
    }

    pub fn proc<F: Fn(HWND, u32, WPARAM, LPARAM) -> bool + Send + Sync + 'static>(
        mut self,
        proc: F,
    ) -> Self {
        self.options.proc = Some(Arc::new(proc));
        self
    }

    pub fn create(self) -> windows::core::Result<Box<Window>> {
        Window::create(self.options)
    }
}

pub struct WindowOptions {
    pub title: HSTRING,
    pub theme: Theme,
    pub proc: Option<Handler>,
    pub class: HSTRING,
    pub show: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: HSTRING::from(""),
            theme: Theme::Auto,
            proc: None,
            class: HSTRING::from(format!("Cypress-Window-{}", uuid::Uuid::new_v4())),
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
    options: WindowOptions,

    theme_cookie: Option<EventRegistrationToken>,
}

pub fn is_dark(color: Color) -> bool {
    ((5 * color.G as u32) + (2 * color.R as u32) + color.B as u32) > (8u32 * 128u32)
}

impl Window {
    pub fn create(options: WindowOptions) -> windows::core::Result<Box<Self>> {
        let mut window = Box::new(Window {
            handle: HWND(0),
            options,
            theme_cookie: None,
        });

        unsafe {
            let instance = GetModuleHandleW(None)?;
            debug_assert!(instance.0 != 0);

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance.into(),
                lpszClassName: window.class().as_pcwstr(),

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
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                Some(&window.options as *const _ as *const _),
            );
        };

        window.set_theme(window.options.theme)?;
        if window.options.show {
            window.show();
        }
        Ok(window)
    }

    pub fn proc(&self) -> Option<&Handler> {
        self.options.proc.as_ref()
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
                            let dark_mode = BOOL((
                                is_dark(UI_SETTINGS.GetColorValue(UIColorType::Foreground).unwrap())
                            ) as i32);
                            DwmSetWindowAttribute(
                                handle,
                                ColorMode::Dark.into(),
                                &dark_mode as *const _ as *const _,
                                4,
                            )
                                .unwrap();
                        }
                        Ok(())
                    }),
                )?);

                let dark_mode =
                    is_dark(UI_SETTINGS.GetColorValue(UIColorType::Foreground).unwrap());
                BOOL(dark_mode as i32)
            },
        };

        unsafe {
            DwmSetWindowAttribute(
                self.handle,
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
