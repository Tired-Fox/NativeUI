use std::{
    collections::HashMap,
    sync::atomic::{AtomicU16, Ordering},
};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use style::{
    color::{hex, Color},
    Prop, BS,
};
pub use windows::{s as pcstr, w as pwstr};

static WIN_ID: AtomicU16 = AtomicU16::new(1);

use crate::{
    control::Control,
    core::{
        constants::{HS::ToHatchStyle, WS::TILED_WINDOW},
        image::icon,
        Brush, ChildType, ControlType, ProcResult, Rect, Renderable, ViewType,
    },
};

pub enum HookType {
    QUIT,
}

#[derive(Default, Debug)]
struct Hooks {
    quit: Option<fn(HWND) -> bool>,
}

#[derive(Debug)]
pub struct WindowStyles {
    window: WINDOW_STYLE,
    class: WNDCLASS_STYLES,
}

#[derive(Debug)]
pub struct Window {
    pub title: HSTRING,
    pub background: HBRUSH,

    pub handle: HWND,
    pub instance: HMODULE,
    pub class: HSTRING,
    pub styles: WindowStyles,

    pub alive: bool,
    pub icon: Option<&'static str>,
    pub rect: Rect,
    pub style: HashMap<String, Prop>,
    hooks: Hooks,
    children: Vec<ChildType>,
}

impl Window {
    fn on_message(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> ProcResult {
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
            _ => return ProcResult::Default
        }
        ProcResult::Success
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
                    // (*this).on_create().ok();
                    return LRESULT(0);
                }
                _ => {
                    let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Self;

                    if !this.is_null() {
                        return match (*this).on_message(message, wparam, lparam) {
                            ProcResult::Success => LRESULT(0),
                            ProcResult::Fail => LRESULT(1),
                            ProcResult::Default => DefWindowProcW(window, message, wparam, lparam),
                        };
                    } else {
                        DefWindowProcW(window, message, wparam, lparam)
                    }
                }
            }
        }
    }

    fn create(&mut self) -> Result<(), String> {
        // Create unique window name from a global window counter
        let id = WIN_ID.swap(WIN_ID.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        self.class = HSTRING::from(format!("NativeUi.rs-{}", id).as_str());

        unsafe {
            self.instance = match GetModuleHandleW(None) {
                Ok(module) => {
                    if module.0 == 0 {
                        return Err("Invalid module handle".to_owned());
                    }
                    module
                }
                Err(_) => return Err("Failed to generate module handle".to_owned()),
            };

            let icon = match self.icon {
                Some(ico) => icon(ico)?.0,
                _ => 0,
            };

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: self.instance,
                lpszClassName: PCWSTR::from_raw(self.class.as_ptr()),
                style: self.styles.class,
                lpfnWndProc: Some(Self::wndproc),
                hIcon: HICON(icon),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            if atom == 0 {
                return Err("Failed to register window class".to_owned());
            }

            let handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR::from_raw(self.class.as_ptr()),
                PCWSTR::from_raw(self.title.as_ptr()),
                self.styles.window,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                self.rect.width().into(),
                self.rect.height().into(),
                None,
                None,
                self.instance,
                Some(self as *mut _ as _),
            );

            if handle.0 == 0 || handle != self.handle {
                return Err("Failed to create new window".to_owned());
            }
        }

        Ok(())
    }

    fn apply_styles(&mut self) -> Result<(), String> {
        if self.style.contains_key("width") {
            match self.style.get("width").unwrap() {
                Prop::PX(pixels) => {
                    self.rect.right = pixels.to_owned();
                }
                Prop::Percent(percent) => {
                    self.rect.right = (1920f32 * percent).round() as i32;
                }
                _ => return Err("Invalid value type for window width".to_owned()),
            };
        }

        if self.style.contains_key("height") {
            match self.style.get("height").unwrap() {
                Prop::PX(pixels) => {
                    self.rect.bottom = pixels.to_owned();
                }
                Prop::Percent(percent) => {
                    self.rect.bottom = (1080f32 * percent).round() as i32;
                }
                _ => return Err("Invalid value type for window width".to_owned()),
            };
        }

        build_background(self)?;

        Ok(())
    }
}

impl Window {
    pub fn new() -> Self {
        Window {
            title: HSTRING::new(),
            background: unsafe { CreateSolidBrush(COLORREF(hex("FFF").into())) },
            class: HSTRING::new(),
            styles: WindowStyles {
                window: TILED_WINDOW,
                class: WNDCLASS_STYLES(0),
            },
            handle: HWND(0),
            instance: HMODULE(0),
            hooks: Hooks { quit: None },
            alive: false,
            icon: None,
            rect: Rect::new(0, 0, 400, 300),
            style: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, path: &'static str) -> Self {
        self.icon = Some(path);
        self
    }

    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.rect.right = width;
        self.rect.bottom = height;
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

    pub fn layout(mut self, children: &mut Vec<ChildType>) -> Self {
        self.children.append(children);
        self
    }

    pub fn init(&mut self) -> Result<(), String> {
        if self.class.to_string_lossy().len() == 0 {
            self.apply_styles()?;
            self.create()?;
            self.show();

            for child in self.children.iter_mut() {
                match child {
                    ChildType::Control(control) => match control {
                        ControlType::Text(text) => {
                            text.create(ViewType::Window(
                                self.handle.clone(),
                                self.instance.clone(),
                            ))?;
                            text.show()
                        }
                        _ => (),
                    },
                }
            }
        }
        Ok(())
    }

    pub fn style(mut self, properties: Vec<(&str, Prop)>) -> Self {
        for pair in properties.iter() {
            self.style.insert(pair.0.to_owned(), pair.1.clone());
        }
        self
    }

    pub fn open(&mut self) -> Result<(), String> {
        self.alive = true;
        self.init()?;

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

fn build_background(window: &mut Window) -> Result<(), String> {
    let mut color: Color = "FFF".into();
    let mut pattern = None;
    let mut apply_background = false;

    if window.style.contains_key("background") {
        match window.style.get("background").unwrap() {
            Prop::Background(c, hatch) => {
                color = c.to_owned();
                match hatch {
                    Some(h) => {
                        pattern = Some(h.to_hatch());
                        apply_background = true;
                    }
                    _ => (),
                }
            }
            _ => return Err("Invalid background values".to_owned()),
        };
    }

    if window.style.contains_key("background-color") {
        match window.style.get("background-color").unwrap() {
            Prop::Color(c) => {
                color = c.to_owned();
                apply_background = true;
            }
            _ => return Err("Invalid background-color color value".to_owned()),
        };
    }

    if window.style.contains_key("background-style") {
        match window.style.get("background-style").unwrap() {
            Prop::BackgroundStyle(style) => match style {
                BS::SOLID => {
                    window.background = Brush::solid(color);
                }
                _ => {
                    window.background = Brush::hatch(color, style.to_hatch());
                }
            },
            _ => return Err("Invalid background-style value".to_owned()),
        };
    } else {
        match pattern {
            Some(p) => window.background = Brush::hatch(color, p),
            _ if apply_background => window.background = Brush::solid(color),
            _ => (),
        }
    }
    Ok(())
}

impl Renderable for Window {
    fn update() -> Result<(), String> {
        Ok(())
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }
}
