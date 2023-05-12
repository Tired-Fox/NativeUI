pub mod CS {
    //! Class Styles
    //! Direct mapping of class style constants from the windows api
    use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_VREDRAW, WNDCLASS_STYLES};

    pub const HREDRAW: WNDCLASS_STYLES = CS_HREDRAW;
    pub const VREDRAW: WNDCLASS_STYLES = CS_VREDRAW;
    pub const DEFAULT: WNDCLASS_STYLES = WNDCLASS_STYLES(0);
}

pub mod WS {
    //! Window Styles
    //! Direct mapping of window style constants from the windows api
    use windows::Win32::UI::WindowsAndMessaging::{
        WINDOW_STYLE, WS_CHILD, WS_TILEDWINDOW, WS_VISIBLE, WS_BORDER,
    };

    pub const TILED_WINDOW: WINDOW_STYLE = WS_TILEDWINDOW;
    pub const VISIBLE: WINDOW_STYLE = WS_VISIBLE;
    pub const CHILD: WINDOW_STYLE = WS_CHILD;
    pub const BORDER: WINDOW_STYLE = WS_BORDER;

    pub mod EX {
        use windows::Win32::UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WS_EX_LAYERED};

        pub const LAYERED: WINDOW_EX_STYLE = WS_EX_LAYERED;
        pub const DEFAULT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0);
    }
}

pub mod HS {
    use windows::Win32::Graphics::Gdi::{
        HATCH_BRUSH_STYLE, HS_BDIAGONAL, HS_CROSS, HS_DIAGCROSS, HS_FDIAGONAL, HS_HORIZONTAL,
        HS_VERTICAL,
    };

    pub const DCROSS: HATCH_BRUSH_STYLE = HS_DIAGCROSS;
    pub const CROSS: HATCH_BRUSH_STYLE = HS_CROSS;
    pub const VERTICAL: HATCH_BRUSH_STYLE = HS_VERTICAL;
    pub const HORIZONTAL: HATCH_BRUSH_STYLE = HS_HORIZONTAL;
    pub const TANGENT: HATCH_BRUSH_STYLE = HS_BDIAGONAL;
    pub const DIAGNOL: HATCH_BRUSH_STYLE = HS_FDIAGONAL;

    pub trait ToHatchStyle {
        fn to_hatch(&self) -> HATCH_BRUSH_STYLE;
    }
}

pub mod WM {
    use windows::Win32::UI::WindowsAndMessaging::{WM_ERASEBKGND, WM_PAINT, WM_SIZE, WM_DESTROY, WM_CLOSE, WM_NCPAINT};

    pub const PAINT: u32 = WM_PAINT;
    pub const NC_PAINT: u32 = WM_NCPAINT;
    pub const SIZE: u32 = WM_SIZE;
    pub const ERASEBKGND: u32 = WM_ERASEBKGND;
    pub const CLOSE: u32 = WM_CLOSE;
    pub const DESTROY: u32 = WM_DESTROY;
    pub const DEFAULT: u32 = 0;
}

pub mod DT {
    use windows::Win32::Graphics::Gdi::{DRAW_TEXT_FORMAT, DT_CENTER, DT_SINGLELINE, DT_VCENTER};

    pub const CENTER: DRAW_TEXT_FORMAT = DT_CENTER;
    pub const DEFAULT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0);
    pub const VCENTER: DRAW_TEXT_FORMAT = DT_VCENTER;
    pub const SINGLELINE: DRAW_TEXT_FORMAT = DT_SINGLELINE;
}
