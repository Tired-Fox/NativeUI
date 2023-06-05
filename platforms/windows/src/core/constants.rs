use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;

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
        WINDOW_STYLE, WS_BORDER, WS_CHILD, WS_TILEDWINDOW, WS_VISIBLE, WS_VSCROLL, WS_HSCROLL,
    };

    pub const TILED_WINDOW: WINDOW_STYLE = WS_TILEDWINDOW;
    pub const VISIBLE: WINDOW_STYLE = WS_VISIBLE;
    pub const CHILD: WINDOW_STYLE = WS_CHILD;
    pub const BORDER: WINDOW_STYLE = WS_BORDER;
    pub const DEFAULT: WINDOW_STYLE = WINDOW_STYLE(0);
    pub const VSCROLL: WINDOW_STYLE = WS_VSCROLL;
    pub const HSCROLL: WINDOW_STYLE = WS_HSCROLL;

    pub mod EX {
        use windows::Win32::UI::WindowsAndMessaging::{WINDOW_EX_STYLE, WS_EX_LAYERED};

        pub const LAYERED: WINDOW_EX_STYLE = WS_EX_LAYERED;
        pub const DEFAULT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0);
    }
}

pub mod HS {
    //! Hatch Styles
    //! Direct mapping of hatch style constants from the windows api
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
    //! Window Message
    //! Direct mapping of window message constants from the windows api
    use windows::Win32::UI::WindowsAndMessaging::{
        WM_CLOSE, WM_CREATE, WM_DESTROY, WM_ERASEBKGND, WM_HSCROLL, WM_MOUSEHWHEEL, WM_MOUSEWHEEL,
        WM_NCPAINT, WM_PAINT, WM_SIZE, WM_VSCROLL,
    };

    pub const CREATE: u32 = WM_CREATE;
    pub const PAINT: u32 = WM_PAINT;
    pub const NC_PAINT: u32 = WM_NCPAINT;
    pub const SIZE: u32 = WM_SIZE;
    pub const ERASEBKGND: u32 = WM_ERASEBKGND;
    pub const CLOSE: u32 = WM_CLOSE;
    pub const DESTROY: u32 = WM_DESTROY;
    pub const DEFAULT: u32 = 0;
    pub const VSCROLL: u32 = WM_VSCROLL;
    pub const HSCROLL: u32 = WM_HSCROLL;
    pub const MOUSEWHEEL: u32 = WM_MOUSEWHEEL;
    pub const MOUSEHWHEEL: u32 = WM_MOUSEHWHEEL;

    pub fn preview(message: u32) -> &'static str {
        match message {
            CREATE => "CREATE",
            PAINT => "PAINT",
            NC_PAINT => "NC_PAINT",
            SIZE => "SIZE",
            ERASEBKGND => "ERASEBKGND",
            CLOSE => "CLOSE",
            DESTROY => "DESTROY",
            VSCROLL => "VSCROLL",
            HSCROLL => "HSCROLL",
            _ => "UNKOWN",
        }
    }
}

pub mod DT {
    //! Draw Text
    //! Direct mapping of draw text format constants from the windows api
    use windows::Win32::Graphics::Gdi::{DRAW_TEXT_FORMAT, DT_CENTER, DT_SINGLELINE, DT_VCENTER};

    pub const CENTER: DRAW_TEXT_FORMAT = DT_CENTER;
    pub const DEFAULT: DRAW_TEXT_FORMAT = DRAW_TEXT_FORMAT(0);
    pub const VCENTER: DRAW_TEXT_FORMAT = DT_VCENTER;
    pub const SINGLELINE: DRAW_TEXT_FORMAT = DT_SINGLELINE;
}

pub mod MK {
    //! Modifier Keys
    use windows::Win32::System::SystemServices::{
        MK_CONTROL, MK_LBUTTON, MK_MBUTTON, MK_RBUTTON, MK_SHIFT, MK_XBUTTON1, MK_XBUTTON2,
        
   };
    pub use windows::Win32::System::SystemServices::MODIFIERKEYS_FLAGS as MODIFIERKEY;

    pub const SHIFT: MODIFIERKEY = MK_SHIFT;
    pub const CTRL: MODIFIERKEY = MK_CONTROL;
    pub const LBUTTON: MODIFIERKEY = MK_LBUTTON;
    pub const RBUTTON: MODIFIERKEY = MK_RBUTTON;
    pub const MBUTTON: MODIFIERKEY = MK_MBUTTON;
    pub const XBUTTON1: MODIFIERKEY = MK_XBUTTON1;
    pub const XBUTTON2: MODIFIERKEY = MK_XBUTTON2;
}

pub mod SBS {
    //! Scrollbar style
    //! Direct mapping of scrollbar style constants from the windows api
    use windows::Win32::UI::WindowsAndMessaging::{SBS_HORZ, SBS_VERT};

    pub const HORZ: i32 = SBS_HORZ;
    pub const VERT: i32 = SBS_VERT;
}

pub mod SB {
    //! Scrollbar Contants
    //! Direct mapping of scrollbar constants from the windows api
    use windows::Win32::UI::WindowsAndMessaging::{
        SB_BOTH, SB_CTL, SB_HORZ, SB_LINEDOWN, SB_LINELEFT, SB_LINERIGHT, SB_LINEUP, SB_PAGEDOWN,
        SB_PAGELEFT, SB_PAGERIGHT, SB_PAGEUP, SB_THUMBPOSITION, SB_THUMBTRACK, SB_TOP, SB_VERT,
    };

    pub use windows::Win32::UI::WindowsAndMessaging::{
        SCROLLBAR_COMMAND as COMMAND, SCROLLBAR_CONSTANTS as CONSTANTS,
    };

    pub const BOTH: CONSTANTS = SB_BOTH;
    pub const HORZ: CONSTANTS = SB_HORZ;
    pub const VERT: CONSTANTS = SB_VERT;
    pub const CTL: CONSTANTS = SB_CTL;

    pub const TOP: COMMAND = SB_TOP;
    pub const BOTTOM: COMMAND = SB_TOP;
    pub const THUMBPOSITION: COMMAND = SB_THUMBPOSITION;
    pub const THUMBTRACK: COMMAND = SB_THUMBTRACK;
    pub const LINEDOWN: COMMAND = SB_LINEDOWN;
    pub const LINEUP: COMMAND = SB_LINEUP;
    pub const LINELEFT: COMMAND = SB_LINELEFT;
    pub const LINERIGHT: COMMAND = SB_LINERIGHT;
    pub const PAGEUP: COMMAND = SB_PAGEUP;
    pub const PAGEDOWN: COMMAND = SB_PAGEDOWN;
    pub const PAGELEFT: COMMAND = SB_PAGELEFT;
    pub const PAGERIGHT: COMMAND = SB_PAGERIGHT;
}
