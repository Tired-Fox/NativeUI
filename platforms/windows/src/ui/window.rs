use windows::{
    core::{HSTRING, PCWSTR},
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use style::color::hex;
use style::Overflow;
pub use windows::{s as pcstr, w as pwstr};

use crate::{
    core::{
        constants::{CS, WM, WS},
        image::icon,
        to_RECT, to_Rect,
    },
    macros::controls,
    ui::{component::{ProcResult, ScrollBar}, Brush},
};

use native_core::{Container, Layout, Rect, Renderable, Child};

use super::component::WindowsComponent;

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

impl Default for WindowStyles {
    fn default() -> Self {
        WindowStyles {
            window: WS::TILED_WINDOW,
            class: CS::DEFAULT,
        }
    }
}

#[derive(Debug)]
pub struct Window {
    index: u32,
    max_point: (i32, i32),
    initialized: bool,
    pub title: HSTRING,
    pub background: HBRUSH,
    id: String,
    classes: Vec<String>,

    pub handle: HWND,
    pub instance: HMODULE,
    pub class: HSTRING,
    pub styles: WindowStyles,

    pub alive: bool,
    pub icon: Option<&'static str>,
    pub rect: Rect,

    pub layout: Layout,
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

                let rect = self.rect().clone();
                self.update(rect)
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

    fn apply_styles(&mut self) -> Result<(), String> {
        let (dimensions, appearance) = self.get_styles();
        self.rect.right = dimensions.width.as_i32(
            1900,
            match self.rect.width() {
                0 => 400,
                _ => self.rect.width(),
            },
        );
        self.rect.bottom = dimensions.height.as_i32(
            1000,
            match self.rect.width() {
                0 => 300,
                _ => self.rect.height(),
            },
        );

        self.background = match appearance.background_color {
            Some(color) => Brush::solid(color),
            None => self.background
        };

        Ok(())
    }
}

pub struct WindowBuilder {
    index: u32,
    id: String,
    classes: Vec<String>,
    title: HSTRING,
    background: HBRUSH,
    rect: Rect,
    class: HSTRING,
    styles: WindowStyles,
    icon: Option<&'static str>,
    layout: Layout,
    hooks: Hooks,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            index: 0,
            id: String::new(),
            classes: vec![String::from("window")],
            title: HSTRING::new(),
            rect: Rect::default(),
            class: HSTRING::new(),
            styles: WindowStyles::default(),
            background: unsafe { CreateSolidBrush(COLORREF(hex("FFF").into())) },
            icon: None,
            layout: Layout::new(),
            hooks: Hooks::default(),
        }
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = index;
        self
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

    pub fn classes(mut self, classes: Vec<String>) -> Self {
        self.classes.extend(classes);
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

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn build(self) -> Window {
        Window {
            index: self.index,
            initialized: false,
            id: self.id,
            classes: self.classes,
            max_point: (self.rect.width(), self.rect.height()),
            background: self.background,
            handle: HWND(0),
            instance: HMODULE(0),
            title: self.title,
            class: self.class,
            styles: self.styles,
            alive: false,
            icon: self.icon,
            rect: self.rect,
            layout: self.layout,
            hooks: self.hooks,
            scrollbars: (ScrollBar::default(), ScrollBar::default()),
        }
    }

    pub fn open(self) -> Result<(), String> {
        self.build().open()
    }
}

impl Window {
    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }

    pub fn new() -> Self {
        Window {
            index: 0,
            initialized: false,
            id: String::new(),
            classes: vec![String::from("window")],
            max_point: (0, 0),
            title: HSTRING::new(),
            background: unsafe { CreateSolidBrush(COLORREF(hex("FFF").into())) },
            class: HSTRING::new(),
            styles: WindowStyles::default(),
            handle: HWND(0),
            instance: HMODULE(0),
            hooks: Hooks { quit: None },
            alive: false,
            icon: None,
            rect: Rect::new(0, 0, 400, 300),
            layout: Layout::new(),
            scrollbars: (ScrollBar::default(), ScrollBar::default()),
        }
    }

    pub fn build(&mut self) -> Result<(), String> {
        if !self.initialized {
            self.apply_styles()?;
            self.init()?;
            self.show();
            self.initialized = true;
        }
        Ok(())
    }

    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    pub fn open(&mut self) -> Result<(), String> {
        self.alive = true;
        self.build()?;

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

impl Container for Window {
    fn layout(&mut self) -> &mut Layout {
        &mut self.layout
    }

    fn init(&mut self) -> Result<(), String> {
        self.class = HSTRING::from(format!("NativeUi.rs-{}", self.index).as_str());

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

            for child in self.layout().children.iter_mut() {
                match child {
                    Child::Component(component) => {
                        let component = &mut *component.borrow_mut();
                        component.create((self.handle, self.instance));
                    },
                    Child::Container(container) => {
                        container.borrow_mut().init();
                    }
                }
            }
        }

        self.scrollbars = (controls::scrollbar!(12, "h"), controls::scrollbar!(12, "v"));

        self.scrollbars.0.create((self.handle, self.instance))?;
        self.scrollbars.1.create((self.handle, self.instance))?;

        let dimensions = self.get_styles().0;
        if dimensions.overflow_x == Overflow::Scroll {
            self.scrollbars.0.show();
        }

        if dimensions.overflow_y == Overflow::Scroll {
            self.scrollbars.1.show();
        }

        Ok(())
    }
}

impl Renderable for Window {
    fn update(&mut self, rect: Rect) {
        let dimensions = self.get_styles().0.clone();
        self.layout.update(&rect, &dimensions);
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn classes(&self) -> &Vec<String> {
        &self.classes
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn default_rect(&self) -> &Rect {
        &self.rect
    }

    fn update_rect(&mut self, rect: Rect) {
        self.rect = rect
    }

    fn show(&mut self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW);
        }
    }

    fn hide(&mut self) {
        unsafe {
            ShowWindow(self.handle, SW_HIDE);
        }
    }
}
