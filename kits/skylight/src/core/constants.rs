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
    use windows::Win32::UI::WindowsAndMessaging::{WINDOW_STYLE, WS_TILEDWINDOW, WS_VISIBLE, WS_CHILD};

    pub const TILED_WINDOW: WINDOW_STYLE = WS_TILEDWINDOW;
    pub const VISIBLE: WINDOW_STYLE = WS_VISIBLE;
    pub const CHILD: WINDOW_STYLE = WS_CHILD;

    pub mod EX {
        use windows::Win32::UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WS_EX_LAYERED};

        pub const LAYERED: WINDOW_EX_STYLE = WS_EX_LAYERED;
        pub const DEFAULT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0);
    }
}

pub mod HS {
    use windows::Win32::Graphics::Gdi::{HATCH_BRUSH_STYLE, HS_DIAGCROSS, HS_CROSS, HS_VERTICAL, HS_HORIZONTAL, HS_BDIAGONAL, HS_FDIAGONAL};
    use style::BS;

    pub const DCROSS: HATCH_BRUSH_STYLE = HS_DIAGCROSS;
    pub const CROSS: HATCH_BRUSH_STYLE = HS_CROSS;
    pub const VERTICAL: HATCH_BRUSH_STYLE = HS_VERTICAL;
    pub const HORIZONTAL: HATCH_BRUSH_STYLE = HS_HORIZONTAL;
    pub const TANGENT: HATCH_BRUSH_STYLE = HS_BDIAGONAL;
    pub const DIAGNOL: HATCH_BRUSH_STYLE = HS_FDIAGONAL;


    pub trait ToHatchStyle {
        fn to_hatch(&self) -> HATCH_BRUSH_STYLE;
    }

    impl ToHatchStyle for BS {
        fn to_hatch(&self) -> HATCH_BRUSH_STYLE {
            match self{
                Self::DCROSS => DCROSS,
                Self::CROSS => CROSS,
                Self::VERTICAL => VERTICAL,
                Self::HORIZONTAL => HORIZONTAL,
                Self::TANGENT => TANGENT,
                Self::DIAGNOL => DIAGNOL,
                _ => HATCH_BRUSH_STYLE(0)
            }
        }
    }
}

pub mod WM {
    use windows::Win32::UI::WindowsAndMessaging::WM_PAINT;

    pub const PAINT: u32 = WM_PAINT;
    pub const DEFAULT: u32 = 0;
}

pub mod DT {
    use windows::Win32::Graphics::Gdi::{DT_CENTER, DRAW_TEXT_FORMAT, DT_VCENTER, DT_SINGLELINE};

    pub const CENTER: DRAW_TEXT_FORMAT = DT_CENTER;
    pub const DEFAULT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0);
    pub const VCENTER: DRAW_TEXT_FORMAT = DT_VCENTER;
    pub const SINGLELINE: DRAW_TEXT_FORMAT = DT_SINGLELINE;
}
