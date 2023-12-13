# Cypress CSS

A css parser, compiler, and minifier inspired by projects like [Parcel](https://parceljs.org/blog/parcel-css/) and [servo](https://github.com/servo/servo/tree/main/components/style), with inspired libraries including [selectors](https://github.com/servo/servo/tree/main/components/selectors) and [cssparser](https://github.com/servo/rust-cssparser).

Uses cssparser under the hood for the generic css parsing. The spec, minification, compilation, and minification is done
manually inside this crate. This crate is meant to keep up with the full browser spec and can be used by any application or library hoping to use a css parser of that calliber.

Parsing is outlined with [CSS Syntaax Module Level 3](https://drafts.csswg.org/css-syntax/).
Example project that sort of does this, but without decleration parsing. [css](https://docs.rs/css/latest/css/)
