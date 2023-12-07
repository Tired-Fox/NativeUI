//! Example syntax from [Parcel CSS](https://parceljs.org/blog/parcel-css/):
//!
//! ```
//! Background([Background {
//!   image: Url(Url { url: "img.png" }),
//!   color: CssColor(RGBA(RGBA { red: 0, green: 0, blue: 0, alpha: 0 })),
//!   position: Position {
//!     x: Length(Dimension(Px(20.0))),
//!     y: Length(Dimension(Px(10.0))),
//!   },
//!   repeat: BackgroundRepeat {
//!     x: Repeat,
//!     y: Repeat,
//!   },
//!   size: Explicit {
//!     width: LengthPercentage(Dimension(Px(50.0))),
//!     height: LengthPercentage(Dimension(Px(100.0))),
//!   },
//!   attachment: Scroll,
//!   origin: PaddingBox,
//!   clip: BorderBox,
//! }])
//! ```
//!
//! Goal Strict Typing and Objects

pub mod tokenizer;
mod parser;

