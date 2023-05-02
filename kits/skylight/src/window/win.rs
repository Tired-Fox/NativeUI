use std::sync::atomic::{AtomicU16, Ordering};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use style::color::hex;
pub use windows::{s as pcstr, w as pwstr};

static WIN_ID: AtomicU16 = AtomicU16::new(1);

use crate::core::image::icon;

pub enum HookType {
    QUIT,
}

#[derive(Default)]
struct Hooks {
    quit: Option<fn(HWND) -> bool>,
}

pub struct WindowStyles {
    window: WINDOW_STYLE,
    class: WNDCLASS_STYLES,
}

pub struct Window {
    pub title: HSTRING,
    pub width: i16,
    pub height: i16,
    pub background: HBRUSH,

    pub handle: HWND,
    pub class: HSTRING,
    pub styles: WindowStyles,

    pub alive: bool,
    pub icon: Option<&'static str>,
    instance: HMODULE,
    hooks: Hooks,
}

impl Window {
    fn on_create(&mut self) -> Result<(), &str> {
        unsafe {
            if SetWindowPos(
                self.handle,
                None,
                0,
                0,
                self.width.into(),
                self.height.into(),
                SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOZORDER,
            )
            .0 == 0
            {
                return Err("Failed to set windows initial size");
            }
            Ok(())
        }
    }

    fn on_message(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_ERASEBKGND => unsafe {
                // Redraw the window background when an erase background event occurs
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(self.handle, &mut ps);
                FillRect(hdc, &ps.rcPaint, self.background);
                EndPaint(self.handle, &ps);
            },
            WM_CLOSE => unsafe {
                // If quit hook is set execute the hook
                match self.hooks.quit {
                    Some(on_quit) => {
                        if on_quit(self.handle) {
                            DestroyWindow(self.handle);
                        }
                    }
                    _ => {
                        DestroyWindow(self.handle);
                    }
                }
            },
            WM_DESTROY => {
                // Mark the window as no longer alive for message loop
                self.alive = false;
            }
            _ => unsafe {
                return DefWindowProcW(self.handle, message, wparam, lparam);
            },
        }
        LRESULT(0)
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match message {
                WM_CREATE => {
                    let cs = lparam.0 as *const CREATESTRUCTA;
                    let this = (*cs).lpCreateParams as *mut Self;
                    (*this).handle = window;

                    SetWindowLongPtrW(window, GWLP_USERDATA, this as _);
                    (*this).on_create().ok();
                    return LRESULT(0);
                }
                _ => {
                    let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Self;

                    if !this.is_null() {
                        return (*this).on_message(message, wparam, lparam);
                    } else {
                        DefWindowProcW(window, message, wparam, lparam)
                    }
                }
            }
        }
    }

    pub fn icon(mut self, path: &'static str) -> Self {
        self.icon = Some(path);
        self
    }

    fn create(&mut self) -> Result<(), &'static str> {
        // Create unique window name from a global window counter
        let id = WIN_ID.swap(WIN_ID.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        self.class = HSTRING::from(format!("NativeUi.rs-{}", id).as_str());

        if true {
            return Err("Invalid scope");
        }

        unsafe {
            let instance = match GetModuleHandleW(None) {
                Ok(module) => {
                    if module.0 == 0 {
                        return Err("Invalid module handle");
                    }
                    module
                }
                Err(_) => return Err("Failed to generate module handle"),
            };

            let icon = match self.icon {
                Some(ico) => icon(ico).0,
                _ => 0,
            };

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance,
                lpszClassName: PCWSTR::from_raw(self.class.as_ptr()),
                style: self.styles.class,
                lpfnWndProc: Some(Self::wndproc),
                hIcon: HICON(icon),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            if atom == 0 {
                return Err("Failed to register window class");
            }

            let handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR::from_raw(self.class.as_ptr()),
                PCWSTR::from_raw(self.title.as_ptr()),
                self.styles.window,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                Some(self as *mut _ as _),
            );

            if handle.0 == 0 || handle != self.handle {
                return Err("Failed to create new window");
            }
        }

        Ok(())
    }
}

impl Window {
    pub fn new() -> Self {
        Window {
            title: HSTRING::new(),
            background: unsafe { CreateSolidBrush(COLORREF(hex("ff4747").into())) },
            class: HSTRING::new(),
            styles: WindowStyles {
                window: WS_TILEDWINDOW,
                class: WNDCLASS_STYLES(0),
            },
            handle: HWND(0),
            width: 400,
            height: 300,
            hooks: Hooks { quit: None },
            alive: false,
            instance: HMODULE(0),
            icon: None,
        }
    }

    pub fn size(mut self, width: i16, height: i16) -> Self {
        self.height = height;
        self.width = width;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = HSTRING::from(title);
        self
    }

    pub fn background(mut self, brush: HBRUSH) -> Self {
        self.background = brush;
        self
    }

    pub fn hook(mut self, event_key: HookType, callback: fn(HWND) -> bool) -> Self {
        match event_key {
            HookType::QUIT => self.hooks.quit = Some(callback),
        }

        self
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        if self.class.to_string_lossy().len() == 0 {
            self.create()?;
        }
        Ok(())
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }
    pub fn open(mut self) -> Result<(), &'static str> {
        self.init()?;
        self.alive = true;
        self.show();

        unsafe {
            let mut message = MSG::default();

            while self.alive {
                GetMessageA(&mut message, self.handle, 0, 0);
                DispatchMessageA(&message);
            }
        }
        Ok(())
    }
}
