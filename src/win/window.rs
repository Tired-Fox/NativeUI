use std::sync::atomic::{AtomicU16, Ordering};
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::WindowsAndMessaging::*,
};

pub use windows::{s as pcstr, w as pwstr};

static WIN_ID: AtomicU16 = AtomicU16::new(1);

use super::core::hex;

#[derive(Default)]
struct Handlers {
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
    handlers: Handlers,
}

impl Window {
    fn on_click(&mut self, lparam: LPARAM) -> Result<()> {
        let x = lparam.0 as u16 as f32;
        let y = (lparam.0 >> 16) as f32;

        if cfg!(debug_assertions) {
            println!("x: {}, y: {}", x, y);
        }

        Ok(())
    }

    fn on_draw(&mut self) -> Result<()> {
        unsafe { ValidateRect(self.handle, None).ok() }
    }

    fn on_create(&mut self) -> Result<()> {
        // println!("CREATE");
        unsafe {
            SetWindowPos(
                self.handle,
                None,
                0,
                0,
                self.width.into(),
                self.height.into(),
                SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOZORDER,
            )
            .ok()
        }
    }

    fn on_message(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_ERASEBKGND => unsafe {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(self.handle, &mut ps);
                FillRect(hdc, &ps.rcPaint, self.background);
                EndPaint(self.handle, &ps);
            },
            WM_CLOSE => unsafe {
                match self.handlers.quit {
                    Some(on_quit) => {
                        if on_quit(self.handle) {
                            DestroyWindow(self.handle);
                        }
                    }
                    _ => {
                        DestroyWindow(self.handle);
                    }
                }
                if let Some(on_quit) = self.handlers.quit {
                    if on_quit(self.handle) {
                        DestroyWindow(self.handle);
                    }
                } else {
                    DestroyWindow(self.handle);
                }
            },
            WM_DESTROY => {
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

    fn create(&mut self) -> Result<()> {
        let id = WIN_ID.swap(WIN_ID.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        self.class = HSTRING::from(format!("NativeUi.rs-{}", id).as_str());

        unsafe {
            let instance = GetModuleHandleW(None)?;
            debug_assert!(instance.0 != 0);

            let icon = match self.icon {
                Some(ico) => icon(ico).0,
                _ => 0,
            };

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance,
                lpszClassName: PCWSTR::from_raw(self.class.as_ptr()),
                style: self.styles.class,
                lpfnWndProc: Some(Self::wndproc),
                hIcon: HICON(icon),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

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

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
        }

        Ok(())
    }
}

impl Window {
    pub fn new() -> Self {
        Window {
            title: HSTRING::new(),
            background: unsafe { CreateSolidBrush(COLORREF(hex("ff4747"))) },
            class: HSTRING::new(),
            styles: WindowStyles {
                window: WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                class: CS_HREDRAW | CS_VREDRAW,
            },
            handle: HWND(0),
            width: 400,
            height: 300,
            handlers: Handlers { quit: None },
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

    pub fn style(mut self, window: WINDOW_STYLE, class: WNDCLASS_STYLES) -> Self {
        self.styles.window = window;
        self.styles.class = class;
        self
    }

    pub fn bind(mut self, event_key: EventKey, callback: fn(HWND) -> bool) -> Self {
        match event_key {
            EventKey::QUIT => self.handlers.quit = Some(callback),
        }

        self
    }

    pub fn init(&mut self) {
        if self.class.to_string_lossy().len() == 0 {
            println!("Generate window");
            self.create().ok();
        } else {
            println!("Don't generate window");
        }
    }

    pub fn open(mut self) -> Self {
        self.init();
        self.alive = true;
        unsafe { ShowWindow(self.handle, SW_SHOW); }

        unsafe {
            let mut message = MSG::default();

            while self.alive {
                GetMessageA(&mut message, self.handle, 0, 0);
                DispatchMessageA(&message);
            }
        }
        self
    }
}

pub fn run(mut windows: Vec<&mut Window>) {
    for win in windows.iter_mut() {
        win.init();
        win.alive = true;
        unsafe { ShowWindow(win.handle, SW_SHOW) };
    }

    unsafe {
        let mut message = MSG::default();

        while windows.iter().any(|e| e.alive) {
            GetMessageA(&mut message, None, 0, 0);
            DispatchMessageA(&message);
        }
    }
}

pub enum EventKey {
    QUIT,
}

pub fn icon(path: &str) -> LOADIMAGE_HANDLE {
    unsafe {
        LoadImageW(
            None,
            PCWSTR(HSTRING::from(path).as_ptr()),
            IMAGE_ICON,
            0,
            0,
            LR_LOADFROMFILE | LR_SHARED | LR_LOADTRANSPARENT | LR_DEFAULTSIZE,
        )
        .unwrap()
    }
}
