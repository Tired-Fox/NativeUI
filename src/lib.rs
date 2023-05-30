use native_core::STYLESHEET;

#[cfg(target_os = "windows")]
pub use skylight;

use style::Stylesheet;

pub mod prelude;
pub mod ui;
pub use core;
pub use style;

pub struct AppBuilder {
    window_count: u32,
    windows: Vec<ui::Window>,
}

impl AppBuilder {
    pub fn set_style(self, stylesheet: Stylesheet) -> Self {
        STYLESHEET.0.write().unwrap().dup(stylesheet);
        self
    }

    pub fn add_window(mut self, mut window: ui::Window) -> Self {
        window.set_index(self.window_count);
        self.window_count += 1;
        self.windows.push(window);
        self
    }

    pub fn add_windows(mut self, windows: Vec<ui::Window>) -> Self {
        for mut window in windows {
            window.set_index(self.window_count);
            self.window_count += 1;
            self.windows.push(window);
        }
        self
    }

    pub fn build(self) -> App {
        App {
            window_count: self.window_count,
            windows: self.windows,
        }
    }

    pub fn run(self) -> Result<(), String> {
        let app = App {
            window_count: self.window_count,
            windows: self.windows,
        };
        app.run()
    }
}

pub struct App {
    window_count: u32,
    windows: Vec<ui::Window>,
}

impl App {
    pub fn new() -> Self {
        App {
            window_count: 0,
            windows: Vec::new(),
        }
    }

    pub fn builder() -> AppBuilder {
        AppBuilder {
            window_count: 0,
            windows: Vec::new(),
        }
    }

    pub fn to_builder(self) -> AppBuilder {
        AppBuilder {
            window_count: self.window_count,
            windows: self.windows,
        }
    }

    pub fn set_style(stylesheet: Stylesheet) {
        STYLESHEET.0.write().unwrap().dup(stylesheet);
    }

    pub fn add_window(&mut self, mut window: ui::Window) {
        window.set_index(self.window_count);
        self.window_count += 1;
        self.windows.push(window)
    }

    pub fn add_windows(&mut self, windows: Vec<ui::Window>) {
        for mut window in windows {
            window.set_index(self.window_count);
            self.window_count += 1;
            self.windows.push(window);
        }
    }

    pub fn run(self) -> Result<(), String> {
        ui::run(self.windows)
    }
}
