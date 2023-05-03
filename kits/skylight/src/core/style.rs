pub mod CS {
    //! Class Styles
    //! Direct mapping of class style constants from the windows api
    use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_VREDRAW, WNDCLASS_STYLES};

    pub const HREDRAW: WNDCLASS_STYLES = CS_HREDRAW;
    pub const VREDRAW: WNDCLASS_STYLES = CS_VREDRAW;
}

pub mod WS {
    //! Window Styles
    //! Direct mapping of window style constants from the windows api
    use windows::Win32::UI::WindowsAndMessaging::{WINDOW_STYLE, WS_TILEDWINDOW, WS_VISIBLE};

    pub const TILED_WINDOW: WINDOW_STYLE = WS_TILEDWINDOW;
    pub const VISIBLE: WINDOW_STYLE = WS_VISIBLE;
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
