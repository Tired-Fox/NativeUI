use std::collections::HashSet;

use windows::{
    core::{HSTRING, PCWSTR},
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use style::color::hex;
pub use windows::{s as pcstr, w as pwstr};

use crate::{
    core::{
        constants::{CS, WM, WS},
        error::{Error, WinError},
        image::icon,
        scroll::{hscroll, resize_scrollbars, vscroll},
        to_RECT, to_Rect, wndproc, Proc, ProcResult,
    },
    ui::Brush,
};

use native_core::{Child, Container, Layout, Rect, Renderable};

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
    pub alive: bool,
    pub icon: Option<&'static str>,

    pub title: HSTRING,
    pub background: HBRUSH,
    id: String,
    classes: HashSet<String>,

    pub handle: HWND,
    pub instance: HMODULE,
    pub class: HSTRING,
    pub styles: WindowStyles,
    pub rect: Rect,

    pub layout: Layout<(HWND, HMODULE), Error>,
    hooks: Hooks,
}

impl Proc for Window {
    fn proc(&mut self, _handle: HWND, message: u32, wparam: WPARAM, _lparam: LPARAM) -> ProcResult {
        match message {
            WM::VSCROLL => {
                vscroll(self.handle, wparam);
            }
            WM::HSCROLL => {
                hscroll(self.handle, wparam);
            }
            WM::SIZE => {
                let mut rect: RECT = to_RECT(Rect::new(0, 0, 0, 0));
                unsafe {
                    GetClientRect(self.handle, &mut rect as *mut RECT);
                    self.rect = to_Rect(rect.into());
                    InvalidateRect(self.handle, Some(&rect as *const RECT), true);
                }

                let rect = to_Rect(rect.into());
                resize_scrollbars(
                    self.handle,
                    &rect,
                    self.get_styles().0,
                    self.update(self.rect.clone()),
                );
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
}

impl Window {
    fn apply_styles(&mut self) -> Result<(), Error> {
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
            None => self.background,
        };

        Ok(())
    }
}

pub struct WindowBuilder {
    index: u32,
    id: String,
    classes: HashSet<String>,
    title: HSTRING,
    background: HBRUSH,
    rect: Rect,
    class: HSTRING,
    styles: WindowStyles,
    icon: Option<&'static str>,
    layout: Layout<(HWND, HMODULE), Error>,
    hooks: Hooks,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            index: 0,
            id: String::new(),
            classes: HashSet::from(["window".to_string()]),
            title: HSTRING::new(),
            rect: Rect::from([400, 300]),
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

    pub fn classes(mut self, classes: Vec<&str>) -> Self {
        self.classes.extend(
            classes
                .iter()
                .map(|c| match c.starts_with(".") {
                    true => c.to_string(),
                    false => format!(".{}", c),
                })
                .collect::<Vec<String>>(),
        );
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.classes.insert(match class.starts_with("#") {
            true => class.to_string(),
            false => format!(".{}", class),
        });
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

    pub fn layout(mut self, layout: Layout<(HWND, HMODULE), Error>) -> Self {
        self.layout = layout;
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = match id.starts_with("#") {
            true => id.to_string(),
            false if id.trim() != "" => format!("#{}", id),
            _ => String::new(),
        };
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
        }
    }

    pub fn open(self) -> Result<(), Error> {
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
            classes: HashSet::from(["window".to_string()]),
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
        }
    }

    pub fn build(&mut self) -> Result<(), Error> {
        if !self.initialized {
            self.apply_styles().unwrap();
            self.init().unwrap();
            self.show();
            self.initialized = true;
        }
        Ok(())
    }

    pub fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    pub fn open(&mut self) -> Result<(), Error> {
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

impl Container<(HWND, HMODULE), Error> for Window {
    fn layout(&mut self) -> &mut Layout<(HWND, HMODULE), Error> {
        &mut self.layout
    }

    fn init(&mut self) -> Result<(), Error> {
        self.class = HSTRING::from(format!("NativeUi.rs-{}", self.index).as_str());

        unsafe {
            self.instance = match GetModuleHandleW(None) {
                Ok(module) => {
                    if module.0 == 0 {
                        return Err("Invalid module handle".into());
                    }
                    module
                }
                Err(_) => return Err("Failed to generate module handle".into()),
            };

            let icon = match self.icon {
                Some(ico) => icon(ico)?.0,
                _ => 0,
            };

            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW).or::<Error>(Err(WinError::last().into()))?,
                hInstance: self.instance,
                lpszClassName: PCWSTR::from_raw(self.class.as_ptr()),
                style: self.styles.class,
                lpfnWndProc: Some(wndproc::<Window>),
                hIcon: HICON(icon),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            if atom == 0 {
                return Err("Failed to register window class".into());
            }

            self.handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR::from_raw(self.class.as_ptr()),
                PCWSTR::from_raw(self.title.as_ptr()),
                self.styles.window,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                self.rect.width(),
                self.rect.height(),
                None,
                None,
                self.instance,
                Some(self as *mut _ as _),
            );

            if self.handle.0 == 0 {
                return Err("Failed to create new window".into());
            }

            for child in self.layout.children.iter() {
                match child {
                    Child::Component(component) => {
                        let component = &mut *component.borrow_mut();
                        component.create((self.handle.clone(), self.instance.clone()))?;
                    }
                    Child::Container(container) => {
                        container.borrow_mut().init()?;
                    }
                }
            }
        }

        self.max_point = (self.rect.width(), self.rect.height());
        Ok(())
    }
}

impl Renderable for Window {
    fn update(&mut self, rect: Rect) -> (i32, i32) {
        let dimensions = self.get_styles().0.clone();
        self.layout.update(&rect, &dimensions)
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn classes(&self) -> &HashSet<String> {
        &self.classes
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn default_rect(&self) -> &Rect {
        &self.rect
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
