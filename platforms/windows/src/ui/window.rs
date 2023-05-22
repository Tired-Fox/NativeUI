use std::sync::atomic::{AtomicU16, Ordering};
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use style::{color::hex, Position};
use style::{Appearance, Dimensions, Overflow, Stylesheet};
pub use windows::{s as pcstr, w as pwstr};

static WIN_ID: AtomicU16 = AtomicU16::new(1);

use crate::{
    control::{Control, ScrollBar},
    core::{
        constants::{CS, WM, WS},
        image::icon,
        Brush, ChildType, ProcResult, Renderable, View, ViewType, to_RECT, to_Rect,
    },
    macros::controls,
};

use native_core::Rect;

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
    pub style: (Dimensions, Appearance),
    pub stylesheet: Stylesheet,
    pub children: Vec<ChildType>,
    hooks: Hooks,
    scrollbars: (ScrollBar, ScrollBar),
}

impl Window {
    fn on_message(&mut self, message: u32, _wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        match message {
            WM::SIZE => {
                let mut rect: RECT = to_RECT(Rect::new(0, 0, 0, 0));
                unsafe {
                    GetClientRect(self.handle, &mut rect as *mut RECT);
                    self.rect = to_Rect(rect.into());
                    InvalidateRect(self.handle, Some(&rect as *const RECT), true);
                }

                self.update((self.rect.clone(), self.style.clone()), None)
                    .unwrap();
            }
            WM::ERASEBKGND | WM::PAINT => unsafe {
                // Redraw the window background when an erase background event occurs
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(self.handle, &mut ps);
                FillRect(hdc, &ps.rcPaint, self.background);
                EndPaint(self.handle, &ps);
            },
            WM::CLOSE => unsafe {
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
            WM::DESTROY => {
                // Mark the window as no longer alive for message loop
                self.alive = false;
            }
            _ => return ProcResult::Default,
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

        self.scrollbars = (controls::scrollbar!(12, "h"), controls::scrollbar!(12, "v"));

        self.scrollbars.0.create(
            ViewType::Window(self.handle, self.instance),
            &self.stylesheet,
        )?;

        self.scrollbars.1.create(
            ViewType::Window(self.handle, self.instance),
            &self.stylesheet,
        )?;

        if self.style.0.overflow_x == Overflow::Scroll {
            self.scrollbars.0.show();
        }

        if self.style.0.overflow_y == Overflow::Scroll {
            self.scrollbars.1.show();
        }

        Ok(())
    }

    fn apply_styles(&mut self) -> Result<(), String> {
        self.rect.right = self.style.0.width.as_i32(1900, 400);
        self.rect.bottom = self.style.0.height.as_i32(1000, 300);

        self.background = Brush::solid(self.style.1.background_color);

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
                window: WS::TILED_WINDOW,
                class: CS::DEFAULT,
            },
            handle: HWND(0),
            instance: HMODULE(0),
            hooks: Hooks { quit: None },
            alive: false,
            icon: None,
            rect: Rect::new(0, 0, 400, 300),
            style: (Dimensions::default(), Appearance::default()),
            stylesheet: Stylesheet::default(),
            children: Vec::new(),
            scrollbars: (ScrollBar::default(), ScrollBar::default()),
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

    pub fn layout(mut self, children: Vec<ChildType>) -> Self {
        for child in children.iter() {
            self.children.push(child.clone())
        }
        self
    }

    pub fn init(&mut self) -> Result<(), String> {
        if self.class.to_string_lossy().len() == 0 {
            self.apply_styles()?;
            self.create()?;

            for child in self.children.iter_mut() {
                match child {
                    ChildType::Control(control) => {
                        let mut control = control.borrow_mut();
                        control.create(
                            ViewType::Window(self.handle.clone(), self.instance.clone()),
                            &self.stylesheet,
                        )?;
                    }
                }
            }

            self.show();
        }
        Ok(())
    }

    pub fn stylesheet(mut self, stylesheet: Stylesheet) -> Self {
        self.stylesheet = stylesheet;
        self.style = self
            .stylesheet
            .get_styles(vec!["root".to_owned(), "window".to_owned()]);
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

impl Renderable for Window {
    fn update(
        &mut self,
        _parent: (Rect, (Dimensions, Appearance)),
        previous: Option<(Rect, (Dimensions, Appearance))>,
    ) -> Result<(), String> {
        let mut previous = previous;

        for child in self.children.iter() {
            match child {
                ChildType::Control(control) => {
                    let mut control = control.borrow_mut();
                    control.update(_parent, previous)?;
                    if control.style().0.position != Position::Absolute {
                        previous = Some((control.rect().clone(), control.style().clone()));
                    }
                }
            }
        }

        Ok(())
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    fn hide(&self) {
        unsafe {
            ShowWindow(self.handle, SW_HIDE);
        }
    }

    fn handle(&self) -> &HWND {
        &self.handle
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn style(&self) -> &(Dimensions, Appearance) {
        &self.style
    }
}

impl View for Window {
    fn children(&mut self) -> &mut Vec<ChildType> {
        &mut self.children
    }
}

// fn build_background(window: &mut Window) -> Result<(), String> {
//     let mut color: Color = "FFF".into();
//     let mut pattern = None;
//     let mut apply_background = false;

//     if window.style.contains_key("background") {
//         match window.style.get("background").unwrap() {
//             Prop::Background(c, hatch) => {
//                 color = c.to_owned();
//                 match hatch {
//                     Some(h) => {
//                         pattern = Some(h.to_hatch());
//                         apply_background = true;
//                     }
//                     _ => (),
//                 }
//             }
//             _ => return Err("Invalid background values".to_owned()),
//         };
//     }

//     if window.style.contains_key("background-color") {
//         match window.style.get("background-color").unwrap() {
//             Prop::Color(c) => {
//                 color = c.to_owned();
//                 apply_background = true;
//             }
//             _ => return Err("Invalid background-color color value".to_owned()),
//         };
//     }

//     if window.style.contains_key("background-style") {
//         match window.style.get("background-style").unwrap() {
//             Prop::BackgroundStyle(style) => match style {
//                 BS::SOLID => {
//                     window.background = Brush::solid(color);
//                 }
//                 _ => {
//                     window.background = Brush::hatch(color, style.to_hatch());
//                 }
//             },
//             _ => return Err("Invalid background-style value".to_owned()),
//         };
//     } else {
//         match pattern {
//             Some(p) => window.background = Brush::hatch(color, p),
//             _ if apply_background => window.background = Brush::solid(color),
//             _ => (),
//         }
//     }
//     Ok(())
// }
