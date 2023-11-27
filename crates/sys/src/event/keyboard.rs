use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{WM_KEYUP, WM_KEYDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP};

#[derive(Debug, Clone)]
pub enum KeyboardEvent {
    KeyDown(Key),
    KeyUp(Key),
    KeyHold(Key),
}

impl From<(u32, WPARAM, LPARAM)> for KeyboardEvent {
    fn from(v: (u32, WPARAM, LPARAM)) -> Self {
        match v.0 {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                if v.2.0 & 1 << 30 == 0 {
                    KeyboardEvent::KeyDown(Key::from(v.1))
                } else {
                    KeyboardEvent::KeyHold(Key::from(v.1))
                }
            }
            WM_KEYUP | WM_SYSKEYUP => KeyboardEvent::KeyUp(Key::from(v.1)),
            _ => panic!("Unknown keyboard event message: {}", v.0),
        }
    }
}

impl KeyboardEvent {
    pub fn message(m: u32) -> bool {
        match m {
            WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => true,
            _ => false
        }
    }
}

macro_rules! vkeys {
    ($name: ident { $($key: ident = $value: expr),* }) => {
        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash)]
        pub enum $name {
            $($key = $value,)*
        }

        impl $name {
            fn from_usize(v: usize) -> Option<Self> {
                match v {
                    $($value => Some($name::$key),)*
                    _ => None
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash)]
pub enum Key {
    Virtual(VirtualKey),
    Char(char),
}

impl From<WPARAM> for Key {
    fn from(v: WPARAM) -> Self {
        VirtualKey::from_usize(v.0)
            .map_or(
                Key::Char(v.0 as u8 as char),
                |v| {
                    Key::Virtual(v)
                },
            )
    }
}

vkeys![
    VirtualKey {
        Cancel = 0x03,
        Back = 0x08,
        Tab = 0x09,
        Clear = 0x0C,
        Return = 0x0D,
        Shift = 0x10,
        Control = 0x11,
        Menu = 0x12,
        Pause = 0x13,
        Capital = 0x14,
        KanaHangul = 0x15,
        Junja = 0x17,
        Final = 0x18,
        Kanji = 0x19,
        Escape = 0x1B,
        Convert = 0x1C,
        NonConvert = 0x1D,
        Accept = 0x1E,
        ModeChange = 0x1F,
        Space = 0x20,
        PageUp = 0x21,
        PageDown = 0x22,
        End = 0x23,
        Home = 0x24,
        Left = 0x25,
        Up = 0x26,
        Right = 0x27,
        Down = 0x28,
        Select = 0x29,
        Print = 0x2A,
        Execute = 0x2B,
        Snapshot = 0x2C,
        Insert = 0x2D,
        Delete = 0x2E,
        Help = 0x2F,
        LWin = 0x5B,
        RWin = 0x5C,
        Apps = 0x5D,
        Sleep = 0x5F,
        NumPad0 = 0x60,
        NumPad1 = 0x61,
        NumPad2 = 0x62,
        NumPad3 = 0x63,
        NumPad4 = 0x64,
        NumPad5 = 0x65,
        NumPad6 = 0x66,
        NumPad7 = 0x67,
        NumPad8 = 0x68,
        NumPad9 = 0x69,
        Multiply = 0x6A,
        Add = 0x6B,
        Separator = 0x6C,
        Subtract = 0x6D,
        Decimal = 0x6E,
        Divide = 0x6F,
        F1 = 0x70,
        F2 = 0x71,
        F3 = 0x72,
        F4 = 0x73,
        F5 = 0x74,
        F6 = 0x75,
        F7 = 0x76,
        F8 = 0x77,
        F9 = 0x78,
        F10 = 0x79,
        F11 = 0x7A,
        F12 = 0x7B,
        F13 = 0x7C,
        F14 = 0x7D,
        F15 = 0x7E,
        F16 = 0x7F,
        F17 = 0x80,
        F18 = 0x81,
        F19 = 0x82,
        F20 = 0x83,
        F21 = 0x84,
        F22 = 0x85,
        F23 = 0x86,
        F24 = 0x87,
        NumLock = 0x90,
        Scroll = 0x91,
        LShift = 0xA0,
        RShift = 0xA1,
        LControl = 0xA2,
        RControl = 0xA3,
        LMenu = 0xA4,
        RMenu = 0xA5,
        BrowserBack = 0xA6,
        BrowserForward = 0xA7,
        BrowserRefresh = 0xA8,
        BrowserStop = 0xA9,
        BrowserSearch = 0xAA,
        BrowserFavorites = 0xAB,
        BrowserHome = 0xAC,
        VolumeMute = 0xAD,
        VolumeDown = 0xAE,
        VolumeUp = 0xAF,
        MediaNext = 0xB0,
        MediaPrev = 0xB1,
        MediaStop = 0xB2,
        MediaPlayPause = 0xB3,
        LaunchMail = 0xB4,
        LaunchMediaSelect = 0xB5,
        LaunchApp1 = 0xB6,
        LaunchApp2 = 0xB7,
        Semicolon = 0xBA,
        Plus = 0xBB,
        Comma = 0xBC,
        Minus = 0xBD,
        Period = 0xBE,
        Slash = 0xBF,
        Tilde = 0xC0,
        LBracket = 0xDB,
        BackSlash = 0xDC,
        RBracket = 0xDD,
        Quote = 0xDE,
        OEM8 = 0xDF,
        OEM102 = 0xE2,
        ProcessKey = 0xE5,
        Packet = 0xE7,
        Attention = 0xF6,
        CrSel = 0xF7,
        ExSel = 0xF8,
        EraseEof = 0xF9,
        Play = 0xFA,
        Zoom = 0xFB,
        NoName = 0xFC,
        Pa1 = 0xFD,
        OEMClear = 0xFE
    }
];
