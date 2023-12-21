use std::fmt::Display;

/// Name of a color like red, green, or blue.
///
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/named-color
#[derive(Debug)]
pub enum Named {
    /// #F0F8FF | rgb(240 248 255)
    AliceBlue,
    /// #FAEBD7 | rgb(250 235 215)
    AntiqueWhite,
    /// #00FFFF | rgb(0 255 255)
    /// Alias for `Cyan`
    Aqua,
    /// #7FFFD4 | rgb(127 255 212)
    Aquamarine,
    /// #F0FFFF | rgb(240 255 255)
    Azure,
    /// #F5F5DC | rgb(245 245 220)
    Beige,
    /// #FFE4C4 | rgb(255 228 196)
    Bisque,
    /// #000000 | rgb(0 0 0)
    Black,
    /// #FFEBCD | rgb(255 235 205)
    BlanchedAlmond,
    /// #0000FF | rgb(0 0 255)
    Blue,
    /// #8A2BE2 | rgb(138 43 226)
    BlueViolet,
    /// #A52A2A | rgb(165 42 42)
    Brown,
    /// #DEB887 | rgb(222 184 135)
    BurlyWood,
    /// #5F9EA0 | rgb(95 158 160)
    CadetBlue,
    /// #7FFF00 | rgb(127 255 0)
    Chartreuse,
    /// #D2691E | rgb(210 105 30)
    Chocolate,
    /// #FF7F50 | rgb(255 127 80)
    Coral,
    /// #6495ED | rgb(100 149 237)
    CornflowerBlue,
    /// #FFF8DC | rgb(255 248 220)
    CornSilk,
    /// #DC143C | rgb(220 20 60)
    Crimson,
    /// #00FFFF | rgb(0 255 255)
    /// Alias of `Aqua`
    Cyan,
    /// #00008B | rgb(0 0 139)
    DarkBlue,
    /// #008B8B | rgb(0 139 139)
    DarkCyan,
    /// #B8860B | rgb(184 134 11)
    DarkGoldenRod,
    /// #A9A9A9 | rgb(169 169 169)
    /// Alias for `DarkGrey`
    DarkGray,
    /// #006400 | rgb(0 100 0)
    DarkGreen,
    /// #A9A9A9 | rgb(169 169 169)
    DarkGrey,
    /// #BDB76B | rgb(189 183 107)
    /// Alias for `DarkGray`
    DarkKhaki,
    /// #8B008B | rgb(139 0 139)
    DarkMagenta,
    /// #556B2F | rgb(85 107 47)
    DarkOliveGreen,
    /// #FF8C00 | rgb(255 140 0)
    DarkOrange,
    /// #9932CC | rgb(153 50 204)
    DarkOrchid,
    /// #8B0000 | rgb(139 0 0)
    DarkRed,
    /// #E9967A | rgb(233 150 122)
    DarkSalmon,
    /// #8FBC8F | rgb(143 188 143)
    DarkSeaGreen,
    /// #483D8B | rgb(72 61 139)
    DarkSlateBlue,
    /// #2F4F4F | rgb(47 79 79)
    /// Alias of `DarkSlateGrey`
    DarkSlateGray,
    /// #2F4F4F | rgb(47 79 79)
    /// Alias of `DarkSlateGray`
    DarkSlateGrey,
    /// #00CED1 | rgb(0 206 209)
    DarkTurquoise,
    /// #9400D3 | rgb(148 0 211)
    DarkViolet,
    /// #FF1493 | rgb(255 20 147)
    DeepPink,
    /// #00BFFF | rgb(0 191 255)
    DeepSkyBlue,
    /// #696969 | rgb(105 105 105)
    /// Alias of `DimGrey`
    DimGray,
    /// #696969 | rgb(105 105 105)
    /// Alias of `DimGray`
    DimGrey,
    /// #1E90FF | rgb(30 144 255)
    DodgerBlue,
    /// #B22222 | rgb(178 34 34)
    FireBrick,
    /// #FFFAF0 | rgb(255 250 240)
    FloralWhite,
    /// #228B22 | rgb(34 139 34)
    ForestGreen,
    /// #FF00FF | rgb(255 0 255)
    /// Alias of `Magenta`
    Fuchsia,
    /// #DCDCDC | rgb(220 220 220)
    Gainsboro,
    /// #F8F8FF | rgb(248 248 255)
    GhostWhite,
    /// #FFD700 | rgb(255 215 0)
    Gold,
    /// #DAA520 | rgb(218 165 32)
    Goldenrod,
    /// #808080 | rgb(128 128 128)
    /// Alias of `Grey`
    Gray,
    /// #008000 | rgb(0 128 0)
    Green,
    /// #ADFF2F | rgb(173 255 47)
    GreenYellow,
    /// #808080 | rgb(128 128 128)
    /// Alias of `Gray`
    Grey,
    /// #F0FFF0 | rgb(240 255 240)
    Honeydew,
    /// #FF69B4 | rgb(255 105 180)
    HotPink,
    /// #CD5C5C | rgb(205 92 92)
    IndianRed,
    /// #4B0082 | rgb(75 0 130)
    Indigo,
    /// #FFFFF0 | rgb(255 255 240)
    Ivory,
    /// #F0E68C | rgb(240 230 140)
    Khaki,
    /// #E6E6FA | rgb(230 230 250)
    Lavender,
    /// #FFF0F5 | rgb(255 240 245)
    LavenderBlush,
    /// #7CFC00 | rgb(124 252 0)
    LawnGreen,
    /// #FFFACD | rgb(255 250 205)
    LemonChiffon,
    /// #ADD8E6 | rgb(173 216 230)
    LightBlue,
    /// #F08080 | rgb(240 128 128)
    LightCoral,
    /// #E0FFFF | rgb(224 255 255)
    LightCyan,
    /// #FAFAD2 | rgb(250 250 210)
    LightGoldenrodYellow,
    /// #D3D3D3 | rgb(211 211 211)
    /// Alias of `LightGrey`
    LightGray,
    /// #90EE90 | rgb(144 238 144)
    LightGreen,
    /// #D3D3D3 | rgb(211 211 211)
    /// Alias of `LightGray`
    LightGrey,
    /// #FFB6C1 | rgb(255 182 193)
    LightPink,
    /// #FFA07A | rgb(255 160 122)
    LightSalmon,
    /// #20B2AA | rgb(32 178 170)
    LightSeaGreen,
    /// #87CEFA | rgb(135 206 250)
    LightSkyBlue,
    /// #778899 | rgb(119 136 153)
    /// Alias of `LightSlateGrey`
    LightSlateGray,
    /// #778899 | rgb(119 136 153)
    /// Alias of `LightSlateGray`
    LightSlateGrey,
    /// #B0C4DE | rgb(176 196 222)
    LightSteelBlue,
    /// #FFFFE0 | rgb(255 255 224)
    LightYellow,
    /// #00FF00 | rgb(0 255 0)
    Lime,
    /// #32CD32 | rgb(50 205 50)
    LimeGreen,
    /// #FAF0E6 | rgb(250 240 230)
    Linen,
    /// #FF00FF | rgb(255 0 255)
    /// Alias of `Fuchsia`
    Magenta,
    /// #800000 | rgb(128 0 0)
    Maroon,
    /// #66CDAA | rgb(102 205 170)
    MediumAquamarine,
    /// #0000CD | rgb(0 0 205)
    MediumBlue,
    /// #BA55D3 | rgb(186 85 211)
    MediumOrchid,
    /// #9370DB | rgb(147 112 219)
    MediumPurple,
    /// #3CB371 | rgb(60 179 113)
    MediumSeaGreen,
    /// #7B68EE | rgb(123 104 238)
    MediumSlateBlue,
    /// #00FA9A | rgb(0 250 154)
    MediumSpringGreen,
    /// #48D1CC | rgb(72 209 204)
    MediumTurquoise,
    /// #C71585 | rgb(199 21 133)
    MediumVioletred,
    /// #191970 | rgb(25 25 112)
    MidnightBlue,
    /// #F5FFFA | rgb(245 255 250)
    MintCream,
    /// #FFE4E1 | rgb(255 228 225)
    MistyRose,
    /// #FFE4B5 | rgb(255 228 181)
    Moccasin,
    /// #FFDEAD | rgb(255 222 173)
    NavajoWhite,
    /// #000080 | rgb(0 0 128)
    Navy,
    /// #FDF5E6 | rgb(253 245 230)
    OldLace,
    /// #808000 | rgb(128 128 0)
    Olive,
    /// #6B8E23 | rgb(107 142 35)
    OliveDrab,
    /// #FFA500 | rgb(255 165 0)
    Orange,
    /// #FF4500 | rgb(255 69 0)
    OrangeRed,
    /// #DA70D6 | rgb(218 112 214)
    Orchid,
    /// #EEE8AA | rgb(238 232 170)
    PaleGoldenrod,
    /// #98FB98 | rgb(152 251 152)
    PaleGreen,
    /// #AFEEEE | rgb(175 238 238)
    PaleTurquoise,
    /// #DB7093 | rgb(219 112 147)
    PaleVioletred,
    /// #FFEFD5 | rgb(255 239 213)
    PapayaWhip,
    /// #FFDAB9 | rgb(255 218 185)
    Peachpuff,
    /// #CD853F | rgb(205 133 63)
    Peru,
    /// #FFC0CB | rgb(255 192 203)
    Pink,
    /// #DDA0DD | rgb(221 160 221)
    Plum,
    /// #B0E0E6 | rgb(176 224 230)
    PowderBlue,
    /// #800080 | rgb(128 0 128)
    Purple,
    /// #663399 | rgb(102 51 153)
    RebeccaPurple,
    /// #FF0000 | rgb(255 0 0)
    Red,
    /// #BC8F8F | rgb(188 143 143)
    RosyBrown,
    /// #4169E1 | rgb(65 105 225)
    RoyalBlue,
    /// #8B4513 | rgb(139 69 19)
    SaddleBrown,
    /// #FA8072 | rgb(250 128 114)
    Salmon,
    /// #F4A460 | rgb(244 164 96)
    SandyBrown,
    /// #2E8B57 | rgb(46 139 87)
    SeaGreen,
    /// #FFF5EE | rgb(255 245 238)
    SeaShell,
    /// #A0522D | rgb(160 82 45)
    Sienna,
    /// #C0C0C0 | rgb(192 192 192)
    Silver,
    /// #87CEEB | rgb(135 206 235)
    SkyBlue,
    /// #6A5ACD | rgb(106 90 205)
    SlateBlue,
    /// #708090 | rgb(112 128 144)
    /// Alias of `SlateGrey`
    SlateGray,
    /// #708090 | rgb(112 128 144)
    /// Alias of `SlateGray`
    SlateGrey,
    /// #FFFAFA | rgb(255 250 250)
    Snow,
    /// #00FF7F | rgb(0 255 127)
    SpringGreen,
    /// #4682B4 | rgb(70 130 180)
    SteelBlue,
    /// #D2B48C | rgb(210 180 140)
    Tan,
    /// #008080 | rgb(0 128 128)
    Teal,
    /// #D8BFD8 | rgb(216 191 216)
    Thistle,
    /// #FF6347 | rgb(255 99 71)
    Tomato,
    // #---00 | rgb(- - - / 0)
    // Where `-` is any value
    //Transparent,
    /// #40E0D0 | rgb(64 224 208)
    Turquoise,
    /// #EE82EE | rgb(238 130 238)
    Violet,
    /// #F5DEB3 | rgb(245 222 179)
    Wheat,
    /// #FFFFFF | rgb(255 255 255)
    White,
    /// #F5F5F5 | rgb(245 245 245)
    WhiteSmoke,
    /// #FFFF00 | rgb(255 255 0)
    Yellow,
    /// #9ACD3 | rgb(154 205 3)
    YellowGreen,
}

impl Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Named::*;
        write!(
            f,
            "{}",
            match self {
                AliceBlue => "aliceblue",
                AntiqueWhite => "antiquewhite",
                Aqua => "aqua",
                Aquamarine => "aquamarine",
                Azure => "azure",
                Beige => "beige",
                Bisque => "bisque",
                Black => "black",
                BlanchedAlmond => "blanchedalmond",
                Blue => "blue",
                BlueViolet => "blueviolet",
                Brown => "brown",
                BurlyWood => "burlywood",
                CadetBlue => "cadetblue",
                Chartreuse => "chartreuse",
                Chocolate => "chocolate",
                Coral => "coral",
                CornflowerBlue => "cornflowerblue",
                CornSilk => "cornsilk",
                Crimson => "crimson",
                Cyan => "cyan",
                DarkBlue => "darkblue",
                DarkCyan => "darkcyan",
                DarkGoldenRod => "darkgoldenrod",
                DarkGray => "darkgray",
                DarkGreen => "darkgreen",
                DarkGrey => "darkgrey",
                DarkKhaki => "darkkhaki",
                DarkMagenta => "darkmagenta",
                DarkOliveGreen => "darkolivegreen",
                DarkOrange => "darkorange",
                DarkOrchid => "darkorchid",
                DarkRed => "darkred",
                DarkSalmon => "darksalmon",
                DarkSeaGreen => "darkseagreen",
                DarkSlateBlue => "darkslateblue",
                DarkSlateGray => "darkslategray",
                DarkSlateGrey => "darkslategrey",
                DarkTurquoise => "darkturquoise",
                DarkViolet => "darkviolet",
                DeepPink => "deeppink",
                DeepSkyBlue => "deepskyblue",
                DimGray => "dimgray",
                DimGrey => "dimgrey",
                DodgerBlue => "dodgerblue",
                FireBrick => "firebrick",
                FloralWhite => "floralwhite",
                ForestGreen => "forestgreen",
                Fuchsia => "fuchsia",
                Gainsboro => "gainsboro",
                GhostWhite => "ghostwhite",
                Gold => "gold",
                Goldenrod => "goldenrod",
                Gray => "gray",
                Green => "green",
                GreenYellow => "greenyellow",
                Grey => "grey",
                Honeydew => "honeydew",
                HotPink => "hotpink",
                IndianRed => "indianred",
                Indigo => "indigo",
                Ivory => "ivory",
                Khaki => "khaki",
                Lavender => "lavender",
                LavenderBlush => "lavenderblush",
                LawnGreen => "lawngreen",
                LemonChiffon => "lemonchiffon",
                LightBlue => "lightblue",
                LightCoral => "lightcoral",
                LightCyan => "lightcyan",
                LightGoldenrodYellow => "lightgoldenrodyellow",
                LightGray => "lightgray",
                LightGreen => "lightgreen",
                LightGrey => "lightgrey",
                LightPink => "lightpink",
                LightSalmon => "lightsalmon",
                LightSeaGreen => "lightseagreen",
                LightSkyBlue => "lightskyblue",
                LightSlateGray => "lightslategray",
                LightSlateGrey => "lightslategrey",
                LightSteelBlue => "lightsteelblue",
                LightYellow => "lightyellow",
                Lime => "lime",
                LimeGreen => "limegreen",
                Linen => "linen",
                Magenta => "magenta",
                Maroon => "maroon",
                MediumAquamarine => "mediumaquamarine",
                MediumBlue => "mediumblue",
                MediumOrchid => "mediumorchid",
                MediumPurple => "mediumpurple",
                MediumSeaGreen => "mediumseagreen",
                MediumSlateBlue => "mediumslateblue",
                MediumSpringGreen => "mediumspringgreen",
                MediumTurquoise => "mediumturquoise",
                MediumVioletred => "mediumvioletred",
                MidnightBlue => "midnightblue",
                MintCream => "mintcream",
                MistyRose => "mistyrose",
                Moccasin => "moccasin",
                NavajoWhite => "navajowhite",
                Navy => "navy",
                OldLace => "oldlace",
                Olive => "olive",
                OliveDrab => "olivedrab",
                Orange => "orange",
                OrangeRed => "orangered",
                Orchid => "orchid",
                PaleGoldenrod => "palegoldenrod",
                PaleGreen => "palegreen",
                PaleTurquoise => "paleturquoise",
                PaleVioletred => "palevioletred",
                PapayaWhip => "papayawhip",
                Peachpuff => "peachpuff",
                Peru => "peru",
                Pink => "pink",
                Plum => "plum",
                PowderBlue => "powderblue",
                Purple => "purple",
                RebeccaPurple => "rebeccapurple",
                Red => "red",
                RosyBrown => "rosybrown",
                RoyalBlue => "royalblue",
                SaddleBrown => "saddlebrown",
                Salmon => "salmon",
                SandyBrown => "sandybrown",
                SeaGreen => "seagreen",
                SeaShell => "seashell",
                Sienna => "sienna",
                Silver => "silver",
                SkyBlue => "skyblue",
                SlateBlue => "slateblue",
                SlateGray => "slategray",
                SlateGrey => "slategrey",
                Snow => "snow",
                SpringGreen => "springgreen",
                SteelBlue => "steelblue",
                Tan => "tan",
                Teal => "teal",
                Thistle => "thistle",
                Tomato => "tomato",
                Turquoise => "turquoise",
                Violet => "violet",
                Wheat => "wheat",
                White => "white",
                WhiteSmoke => "whitesmoke",
                Yellow => "yellow",
                YellowGreen => "yellowgreen",
            }
        )
    }
}

impl Named {
    pub fn parse(name: &str) -> Option<Self> {
        use Named::*;
        Some(match name {
            "aliceblue" => AliceBlue,
            "antiquewhite" => AntiqueWhite,
            "aqua" => Aqua,
            "aquamarine" => Aquamarine,
            "azure" => Azure,
            "beige" => Beige,
            "bisque" => Bisque,
            "black" => Black,
            "blanchedalmond" => BlanchedAlmond,
            "blue" => Blue,
            "blueviolet" => BlueViolet,
            "brown" => Brown,
            "burlywood" => BurlyWood,
            "cadetblue" => CadetBlue,
            "chartreuse" => Chartreuse,
            "chocolate" => Chocolate,
            "coral" => Coral,
            "cornflowerblue" => CornflowerBlue,
            "cornsilk" => CornSilk,
            "crimson" => Crimson,
            "cyan" => Cyan,
            "darkblue" => DarkBlue,
            "darkcyan" => DarkCyan,
            "darkgoldenrod" => DarkGoldenRod,
            "darkgray" => DarkGray,
            "darkgreen" => DarkGreen,
            "darkgrey" => DarkGrey,
            "darkkhaki" => DarkKhaki,
            "darkmagenta" => DarkMagenta,
            "darkolivegreen" => DarkOliveGreen,
            "darkorange" => DarkOrange,
            "darkorchid" => DarkOrchid,
            "darkred" => DarkRed,
            "darksalmon" => DarkSalmon,
            "darkseagreen" => DarkSeaGreen,
            "darkslateblue" => DarkSlateBlue,
            "darkslategray" => DarkSlateGray,
            "darkslategrey" => DarkSlateGrey,
            "darkturquoise" => DarkTurquoise,
            "darkviolet" => DarkViolet,
            "deeppink" => DeepPink,
            "deepskyblue" => DeepSkyBlue,
            "dimgray" => DimGray,
            "dimgrey" => DimGrey,
            "dodgerblue" => DodgerBlue,
            "firebrick" => FireBrick,
            "floralwhite" => FloralWhite,
            "forestgreen" => ForestGreen,
            "fuchsia" => Fuchsia,
            "gainsboro" => Gainsboro,
            "ghostwhite" => GhostWhite,
            "gold" => Gold,
            "goldenrod" => Goldenrod,
            "gray" => Gray,
            "green" => Green,
            "greenyellow" => GreenYellow,
            "grey" => Grey,
            "honeydew" => Honeydew,
            "hotpink" => HotPink,
            "indianred" => IndianRed,
            "indigo" => Indigo,
            "ivory" => Ivory,
            "khaki" => Khaki,
            "lavender" => Lavender,
            "lavenderblush" => LavenderBlush,
            "lawngreen" => LawnGreen,
            "lemonchiffon" => LemonChiffon,
            "lightblue" => LightBlue,
            "lightcoral" => LightCoral,
            "lightcyan" => LightCyan,
            "lightgoldenrodyellow" => LightGoldenrodYellow,
            "lightgray" => LightGray,
            "lightgreen" => LightGreen,
            "lightgrey" => LightGrey,
            "lightpink" => LightPink,
            "lightsalmon" => LightSalmon,
            "lightseagreen" => LightSeaGreen,
            "lightskyblue" => LightSkyBlue,
            "lightslategray" => LightSlateGray,
            "lightslategrey" => LightSlateGrey,
            "lightsteelblue" => LightSteelBlue,
            "lightyellow" => LightYellow,
            "lime" => Lime,
            "limegreen" => LimeGreen,
            "linen" => Linen,
            "magenta" => Magenta,
            "maroon" => Maroon,
            "mediumaquamarine" => MediumAquamarine,
            "mediumblue" => MediumBlue,
            "mediumorchid" => MediumOrchid,
            "mediumpurple" => MediumPurple,
            "mediumseagreen" => MediumSeaGreen,
            "mediumslateblue" => MediumSlateBlue,
            "mediumspringgreen" => MediumSpringGreen,
            "mediumturquoise" => MediumTurquoise,
            "mediumvioletred" => MediumVioletred,
            "midnightblue" => MidnightBlue,
            "mintcream" => MintCream,
            "mistyrose" => MistyRose,
            "moccasin" => Moccasin,
            "navajowhite" => NavajoWhite,
            "navy" => Navy,
            "oldlace" => OldLace,
            "olive" => Olive,
            "olivedrab" => OliveDrab,
            "orange" => Orange,
            "orangered" => OrangeRed,
            "orchid" => Orchid,
            "palegoldenrod" => PaleGoldenrod,
            "palegreen" => PaleGreen,
            "paleturquoise" => PaleTurquoise,
            "palevioletred" => PaleVioletred,
            "papayawhip" => PapayaWhip,
            "peachpuff" => Peachpuff,
            "peru" => Peru,
            "pink" => Pink,
            "plum" => Plum,
            "powderblue" => PowderBlue,
            "purple" => Purple,
            "rebeccapurple" => RebeccaPurple,
            "red" => Red,
            "rosybrown" => RosyBrown,
            "royalblue" => RoyalBlue,
            "saddlebrown" => SaddleBrown,
            "salmon" => Salmon,
            "sandybrown" => SandyBrown,
            "seagreen" => SeaGreen,
            "seashell" => SeaShell,
            "sienna" => Sienna,
            "silver" => Silver,
            "skyblue" => SkyBlue,
            "slateblue" => SlateBlue,
            "slategray" => SlateGray,
            "slategrey" => SlateGrey,
            "snow" => Snow,
            "springgreen" => SpringGreen,
            "steelblue" => SteelBlue,
            "tan" => Tan,
            "teal" => Teal,
            "thistle" => Thistle,
            "tomato" => Tomato,
            "turquoise" => Turquoise,
            "violet" => Violet,
            "wheat" => Wheat,
            "white" => White,
            "whitesmoke" => WhiteSmoke,
            "yellow" => Yellow,
            "yellowgreen" => YellowGreen,
            _ => return None,
        })
    }
}

impl From<Named> for u32 {
    fn from(value: Named) -> Self {
        use Named::*;
        match value {
            AliceBlue => 15792383,
            AntiqueWhite => 16444375,
            Aqua => 65535,
            Aquamarine => 8388564,
            Azure => 15794175,
            Beige => 16119260,
            Bisque => 16770244,
            Black => 0,
            BlanchedAlmond => 16772045,
            Blue => 255,
            BlueViolet => 9055202,
            Brown => 10824234,
            BurlyWood => 14596231,
            CadetBlue => 6266528,
            Chartreuse => 8388352,
            Chocolate => 13789470,
            Coral => 16744272,
            CornflowerBlue => 6591981,
            CornSilk => 16775388,
            Crimson => 14423100,
            Cyan => 65535,
            DarkBlue => 139,
            DarkCyan => 35723,
            DarkGoldenRod => 12092939,
            DarkGray => 11119017,
            DarkGreen => 25600,
            DarkGrey => 11119017,
            DarkKhaki => 12433259,
            DarkMagenta => 9109643,
            DarkOliveGreen => 5597999,
            DarkOrange => 16747520,
            DarkOrchid => 10040012,
            DarkRed => 9109504,
            DarkSalmon => 15308410,
            DarkSeaGreen => 9419919,
            DarkSlateBlue => 4734347,
            DarkSlateGray => 3100495,
            DarkSlateGrey => 3100495,
            DarkTurquoise => 52945,
            DarkViolet => 9699539,
            DeepPink => 16716947,
            DeepSkyBlue => 49151,
            DimGray => 6908265,
            DimGrey => 6908265,
            DodgerBlue => 2003199,
            FireBrick => 11674146,
            FloralWhite => 16775920,
            ForestGreen => 2263842,
            Fuchsia => 16711935,
            Gainsboro => 14474460,
            GhostWhite => 16316671,
            Gold => 16766720,
            Goldenrod => 14329120,
            Gray => 8421504,
            Green => 32768,
            GreenYellow => 11403055,
            Grey => 8421504,
            Honeydew => 15794160,
            HotPink => 16738740,
            IndianRed => 13458524,
            Indigo => 4915330,
            Ivory => 16777200,
            Khaki => 15787660,
            Lavender => 15132410,
            LavenderBlush => 16773365,
            LawnGreen => 8190976,
            LemonChiffon => 16775885,
            LightBlue => 11393254,
            LightCoral => 15761536,
            LightCyan => 14745599,
            LightGoldenrodYellow => 16448210,
            LightGray => 13882323,
            LightGreen => 9498256,
            LightGrey => 13882323,
            LightPink => 16758465,
            LightSalmon => 16752762,
            LightSeaGreen => 2142890,
            LightSkyBlue => 8900346,
            LightSlateGray => 7833753,
            LightSlateGrey => 7833753,
            LightSteelBlue => 11584734,
            LightYellow => 16777184,
            Lime => 65280,
            LimeGreen => 3329330,
            Linen => 16445670,
            Magenta => 16711935,
            Maroon => 8388608,
            MediumAquamarine => 6737322,
            MediumBlue => 205,
            MediumOrchid => 12211667,
            MediumPurple => 9662683,
            MediumSeaGreen => 3978097,
            MediumSlateBlue => 8087790,
            MediumSpringGreen => 64154,
            MediumTurquoise => 4772300,
            MediumVioletred => 13047173,
            MidnightBlue => 1644912,
            MintCream => 16121850,
            MistyRose => 16770273,
            Moccasin => 16770229,
            NavajoWhite => 16768685,
            Navy => 128,
            OldLace => 16643558,
            Olive => 8421376,
            OliveDrab => 7048739,
            Orange => 16753920,
            OrangeRed => 16729344,
            Orchid => 14315734,
            PaleGoldenrod => 15657130,
            PaleGreen => 10025880,
            PaleTurquoise => 11529966,
            PaleVioletred => 14381203,
            PapayaWhip => 16773077,
            Peachpuff => 16767673,
            Peru => 13468991,
            Pink => 16761035,
            Plum => 14524637,
            PowderBlue => 11591910,
            Purple => 8388736,
            RebeccaPurple => 6697881,
            Red => 16711680,
            RosyBrown => 12357519,
            RoyalBlue => 4286945,
            SaddleBrown => 9127187,
            Salmon => 16416882,
            SandyBrown => 16032864,
            SeaGreen => 3050327,
            SeaShell => 16774638,
            Sienna => 10506797,
            Silver => 12632256,
            SkyBlue => 8900331,
            SlateBlue => 6970061,
            SlateGray => 7372944,
            SlateGrey => 7372944,
            Snow => 16775930,
            SpringGreen => 65407,
            SteelBlue => 4620980,
            Tan => 13808780,
            Teal => 32896,
            Thistle => 14204888,
            Tomato => 16737095,
            Turquoise => 4251856,
            Violet => 15631086,
            Wheat => 16113331,
            White => 16777215,
            WhiteSmoke => 16119285,
            Yellow => 16776960,
            YellowGreen => 634067,
        }
    }
}

impl From<u32> for Named {
    fn from(value: u32) -> Self {
        use Named::*;

        match value {
            15792383 => AliceBlue,
            16444375 => AntiqueWhite,
            65535 => Aqua,
            8388564 => Aquamarine,
            15794175 => Azure,
            16119260 => Beige,
            16770244 => Bisque,
            0 => Black,
            16772045 => BlanchedAlmond,
            255 => Blue,
            9055202 => BlueViolet,
            10824234 => Brown,
            14596231 => BurlyWood,
            6266528 => CadetBlue,
            8388352 => Chartreuse,
            13789470 => Chocolate,
            16744272 => Coral,
            6591981 => CornflowerBlue,
            16775388 => CornSilk,
            14423100 => Crimson,
            139 => DarkBlue,
            35723 => DarkCyan,
            12092939 => DarkGoldenRod,
            11119017 => DarkGray,
            25600 => DarkGreen,
            12433259 => DarkKhaki,
            9109643 => DarkMagenta,
            5597999 => DarkOliveGreen,
            16747520 => DarkOrange,
            10040012 => DarkOrchid,
            9109504 => DarkRed,
            15308410 => DarkSalmon,
            9419919 => DarkSeaGreen,
            4734347 => DarkSlateBlue,
            3100495 => DarkSlateGray,
            52945 => DarkTurquoise,
            9699539 => DarkViolet,
            16716947 => DeepPink,
            49151 => DeepSkyBlue,
            6908265 => DimGray,
            2003199 => DodgerBlue,
            11674146 => FireBrick,
            16775920 => FloralWhite,
            2263842 => ForestGreen,
            16711935 => Fuchsia,
            14474460 => Gainsboro,
            16316671 => GhostWhite,
            16766720 => Gold,
            14329120 => Goldenrod,
            8421504 => Gray,
            32768 => Green,
            11403055 => GreenYellow,
            15794160 => Honeydew,
            16738740 => HotPink,
            13458524 => IndianRed,
            4915330 => Indigo,
            16777200 => Ivory,
            15787660 => Khaki,
            15132410 => Lavender,
            16773365 => LavenderBlush,
            8190976 => LawnGreen,
            16775885 => LemonChiffon,
            11393254 => LightBlue,
            15761536 => LightCoral,
            14745599 => LightCyan,
            16448210 => LightGoldenrodYellow,
            13882323 => LightGray,
            9498256 => LightGreen,
            16758465 => LightPink,
            16752762 => LightSalmon,
            2142890 => LightSeaGreen,
            8900346 => LightSkyBlue,
            7833753 => LightSlateGray,
            11584734 => LightSteelBlue,
            16777184 => LightYellow,
            65280 => Lime,
            3329330 => LimeGreen,
            16445670 => Linen,
            8388608 => Maroon,
            6737322 => MediumAquamarine,
            205 => MediumBlue,
            12211667 => MediumOrchid,
            9662683 => MediumPurple,
            3978097 => MediumSeaGreen,
            8087790 => MediumSlateBlue,
            64154 => MediumSpringGreen,
            4772300 => MediumTurquoise,
            13047173 => MediumVioletred,
            1644912 => MidnightBlue,
            16121850 => MintCream,
            16770273 => MistyRose,
            16770229 => Moccasin,
            16768685 => NavajoWhite,
            128 => Navy,
            16643558 => OldLace,
            8421376 => Olive,
            7048739 => OliveDrab,
            16753920 => Orange,
            16729344 => OrangeRed,
            14315734 => Orchid,
            15657130 => PaleGoldenrod,
            10025880 => PaleGreen,
            11529966 => PaleTurquoise,
            14381203 => PaleVioletred,
            16773077 => PapayaWhip,
            16767673 => Peachpuff,
            13468991 => Peru,
            16761035 => Pink,
            14524637 => Plum,
            11591910 => PowderBlue,
            8388736 => Purple,
            6697881 => RebeccaPurple,
            16711680 => Red,
            12357519 => RosyBrown,
            4286945 => RoyalBlue,
            9127187 => SaddleBrown,
            16416882 => Salmon,
            16032864 => SandyBrown,
            3050327 => SeaGreen,
            16774638 => SeaShell,
            10506797 => Sienna,
            12632256 => Silver,
            8900331 => SkyBlue,
            6970061 => SlateBlue,
            7372944 => SlateGray,
            16775930 => Snow,
            65407 => SpringGreen,
            4620980 => SteelBlue,
            13808780 => Tan,
            32896 => Teal,
            14204888 => Thistle,
            16737095 => Tomato,
            4251856 => Turquoise,
            15631086 => Violet,
            16113331 => Wheat,
            16777215 => White,
            16119285 => WhiteSmoke,
            16776960 => Yellow,
            634067 => YellowGreen,
            color => panic!("Invalid u32 to convert to named color: was {}", color),
        }
    }
}
