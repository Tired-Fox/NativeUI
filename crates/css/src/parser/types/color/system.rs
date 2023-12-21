use std::fmt::Display;

/// Reflects the default color choices used for different parts of the page
///
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/system-color
#[derive(Debug)]
pub enum System {
    AccentColor,
    AccentColorText,
    ActiveText,
    ButtonBorder,
    ButtonFace,
    ButtonText,
    Canvas,
    CanvasText,
    Field,
    FieldText,
    GrayText,
    Highlight,
    LinkText,
    Mark,
    MarkText,
    VisitedText,

    #[deprecated]
    ActiveBorder,
    #[deprecated]
    ActiveCatpion,
    #[deprecated]
    AppWorkspace,
    #[deprecated]
    Background,
    #[deprecated]
    ButtonHighlight,
    #[deprecated]
    ButtonShadow,
    #[deprecated]
    CaptionText,
    #[deprecated]
    InactiveBorder,
    #[deprecated]
    InactiveCaption,
    #[deprecated]
    InactiveCaptionText,
    #[deprecated]
    InfoBackground,
    #[deprecated]
    InfoText,
    #[deprecated]
    Menu,
    #[deprecated]
    MenuText,
    #[deprecated]
    Scrollbar,
    #[deprecated]
    ThreeDDarkShadow,
    #[deprecated]
    ThreeDFace,
    #[deprecated]
    ThreeDHighlight,
    #[deprecated]
    ThreeDLightShadow,
    #[deprecated]
    ThreeDShadow,
    #[deprecated]
    Window,
    #[deprecated]
    WindowFrame,
    #[deprecated]
    WindowText,
}

impl Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use System::*;
        write!(
            f,
            "{}",
            match self {
                System::AccentColor => "accentcolor",
                System::AccentColorText => "accentcolortext",
                System::ActiveText => "activetext",
                System::ButtonBorder => "buttonborder",
                System::ButtonFace => "buttonface",
                System::ButtonText => "buttontext",
                System::Canvas => "canvas",
                System::CanvasText => "canvastext",
                System::Field => "field",
                System::FieldText => "fieldtext",
                System::GrayText => "graytext",
                System::Highlight => "highlight",
                System::LinkText => "linktext",
                System::Mark => "mark",
                System::MarkText => "marktext",
                System::VisitedText => "visitedtext",
                System::ActiveBorder => "activeborder",
                System::ActiveCatpion => "activecatpion",
                System::AppWorkspace => "appworkspace",
                System::Background => "background",
                System::ButtonHighlight => "buttonhighlight",
                System::ButtonShadow => "buttonshadow",
                System::CaptionText => "captiontext",
                System::InactiveBorder => "inactiveborder",
                System::InactiveCaption => "inactivecaption",
                System::InactiveCaptionText => "inactivecaptiontext",
                System::InfoBackground => "infobackground",
                System::InfoText => "infotext",
                System::Menu => "menu",
                System::MenuText => "menutext",
                System::Scrollbar => "scrollbar",
                System::ThreeDDarkShadow => "threeddarkshadow",
                System::ThreeDFace => "threedface",
                System::ThreeDHighlight => "threedhighlight",
                System::ThreeDLightShadow => "threedlightshadow",
                System::ThreeDShadow => "threedshadow",
                System::Window => "window",
                System::WindowFrame => "windowframe",
                System::WindowText => "windowtext",
            }
        )
    }
}

impl System {
    pub fn parse(name: &str) -> Option<Self> {
        Some(match name.to_ascii_lowercase().as_str() {
            "accentcolor" => System::AccentColor,
            "accentcolortext" => System::AccentColorText,
            "activetext" => System::ActiveText,
            "buttonborder" => System::ButtonBorder,
            "buttonface" => System::ButtonFace,
            "buttontext" => System::ButtonText,
            "canvas" => System::Canvas,
            "canvastext" => System::CanvasText,
            "field" => System::Field,
            "fieldtext" => System::FieldText,
            "graytext" => System::GrayText,
            "highlight" => System::Highlight,
            "linktext" => System::LinkText,
            "mark" => System::Mark,
            "marktext" => System::MarkText,
            "visitedtext" => System::VisitedText,
            // Deprecated
            "activeborder" => System::ActiveBorder,
            "activecatpion" => System::ActiveCatpion,
            "appworkspace" => System::AppWorkspace,
            "background" => System::Background,
            "buttonhighlight" => System::ButtonHighlight,
            "buttonshadow" => System::ButtonShadow,
            "captiontext" => System::CaptionText,
            "inactiveborder" => System::InactiveBorder,
            "inactivecaption" => System::InactiveCaption,
            "inactivecaptiontext" => System::InactiveCaptionText,
            "infobackground" => System::InfoBackground,
            "infotext" => System::InfoText,
            "menu" => System::Menu,
            "menutext" => System::MenuText,
            "scrollbar" => System::Scrollbar,
            "threeddarkshadow" => System::ThreeDDarkShadow,
            "threedface" => System::ThreeDFace,
            "threedhighlight" => System::ThreeDHighlight,
            "threedlightshadow" => System::ThreeDLightShadow,
            "threedshadow" => System::ThreeDShadow,
            "window" => System::Window,
            "windowframe" => System::WindowFrame,
            "windowtext" => System::WindowText,
            _ => return None,
        })
    }
}
