use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

use windows::core::{HSTRING, IInspectable};
use windows::Foundation::TypedEventHandler;
use windows::UI::Color;
use windows::UI::ViewManagement::{UIColorType, UISettings};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, LoadCursorW, RegisterClassW, ShowWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, SW_SHOWNORMAL, WINDOW_EX_STYLE, WNDCLASSW, WS_OVERLAPPEDWINDOW, CloseWindow};
use crate::windows::event::watch;

use super::{event::wnd_proc, IntoPCWSTR, is_dark_theme, is_light_theme};

enum ColorMode {
    Light = 0,
    Dark = 20,
}

impl From<ColorMode> for DWMWINDOWATTRIBUTE {
    fn from(color_mode: ColorMode) -> Self {
        DWMWINDOWATTRIBUTE(color_mode as i32)
    }
}

pub struct Window {
    handle: HWND,
    cls: HSTRING,
    title: HSTRING,
}

pub struct Handle(JoinHandle<windows::core::Result<()>>, HWND);
impl Handle {
    pub fn join(self) -> windows::core::Result<()> {
        self.0.join().unwrap()
    }
    pub fn handle(&self) -> HWND {
        self.1
    }
}

pub fn is_dark(color: Color) -> bool {
    ((5*color.G as u32) + (2*color.R as u32) + color.B as u32) > (8u32 * 128u32)
}

impl Window {
    pub fn create(title: &str) -> windows::core::Result<Self> {
        let cls = HSTRING::from("Cypress-Sys");
        let title = HSTRING::from(title);

        let window = unsafe {
            let instance = GetModuleHandleW(None)?;
            debug_assert!(instance.0 != 0);

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance.into(),
                lpszClassName: cls.as_pcwstr(),

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wnd_proc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                cls.as_pcwstr(),
                title.as_pcwstr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                None,
            );

            // Enable light & dark system theme and hook into when it changes
            let settings = UISettings::new().unwrap();
            let forground = settings.GetColorValue(UIColorType::Foreground).unwrap();
            let dark_mode = BOOL((is_dark(forground)) as i32);
            println!("Dark mode: {}", dark_mode.0);
            DwmSetWindowAttribute(
                handle,
                ColorMode::Dark.into(),
                &dark_mode as *const _ as *const c_void,
                4,
            )?;
            settings.ColorValuesChanged(&TypedEventHandler::new(
                move |settings: &Option<UISettings>, _| {
                    if let Some(settings) = settings {
                        let forground = settings.GetColorValue(UIColorType::Foreground).unwrap();
                        let dark_mode = BOOL((is_dark(forground)) as i32);
                        println!("Dark mode: {}", dark_mode.0);
                        DwmSetWindowAttribute(
                            handle,
                            ColorMode::Dark.into(),
                            &dark_mode as *const _ as *const c_void,
                            4,
                        ).unwrap();
                        UpdateWindow(handle);
                    }
                    Ok(())
                }
            ))?;

            Window { handle, cls, title }
        };

        Ok(window)
    }

    /// Show the window
    pub fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    /// Hide the window
    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.handle, SW_HIDE);
        }
    }

    /// Minimize the window
    pub fn minimize(&self) {
        unsafe {
            ShowWindow(self.handle, SW_MINIMIZE);
        }
    }

    /// Restore the window
    pub fn restore(&self) {
        unsafe {
            ShowWindow(self.handle, SW_RESTORE);
        }
    }

    /// Maximize the window
    pub fn maximize(&self) {
        unsafe {
            ShowWindow(self.handle, SW_MAXIMIZE);
        }
    }

    pub fn update(&self) {
        unsafe {
            UpdateWindow(self.handle);
        }
    }

    pub fn close(&self) -> windows::core::Result<()> {
        unsafe {
            CloseWindow(self.handle)
        }
    }

    pub fn handle(&self) -> HWND {
        self.handle
    }

    pub fn class(&self) -> String {
        self.cls.to_string_lossy()
    }

    pub fn title(&self) -> String {
        self.title.to_string_lossy()
    }
}
