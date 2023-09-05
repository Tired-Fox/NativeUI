<div align="center">

![Native UI Logo](assets/images/Native2.png)

</div>

# NativeUI 

<!-- Header Badges -->

<div align="center">
  
<img src="assets/badges/version.svg" alt="Version"/>
<a href="https://github.com/Tired-Fox/NativeUI/releases" alt="Release"><img src="https://img.shields.io/github/v/release/tired-fox/NativeUI.svg?style=flat-square&color=9cf"/></a>
<a href="https://github.com/Tired-Fox/NativeUI/blob/main/LICENSE" alt="License"><img src="assets/badges/license.svg"/></a>
<br>
<img src="assets/badges/maintained.svg" alt="Maintained"/>
<img src="assets/badges/tests.svg" alt="Tests"/>
  
</div>

<!-- End Header -->

## Code Names
  - Apple API  : Braeburn
  - Linux API  : Humboldt
  - Windows API: Skylight

## Core
- Each sub crate wraps the corresponding API's to the best of it's ability. This allows for these sub crates to be used independantly from the overall projec.

> Note: Win32 already has bindings for it's API, The wrapper would be re-exporting and renaming most of the API along with making functionality easier to use.

- The high level library will then define an API for how it can style, structure, and render applications and the translation layer fills in the functionality based on the platform.

- **Structure:**
+ Framework 
  + HTML
    + Parser
    + DOM
      + Access
      + Manipulation
    + Ruleset
  + CSS Parser
    + Parser
    + L3 Ruleset
    + Query
    + Get
  + JS Runtime???
    + API for interaction
    + WASM Runtime
+ API Library
  + Translation Layer
    + WinAPI
    + MacAPI
    + LinuxAPI


## Rendering
- Canvas (Window)
- Meta
- Body
- Syntax tree, node tree, or object tree
- Dynamic and robust layout system
- Occurs on update or on frame tick

## Styling
- Full web spec of CSS (L3), but only certain styles are used.

## Tools
- CSS Parser:
  + Custom
  + [cssparser (Servo)](https://docs.rs/cssparser/latest/cssparser/) with similar spec structure as Servo
- HTML Parser:
  + Custom
  + [html5ever (Servo)](https://github.com/servo/html5ever)
- JS Runtime:
  + [Deno](https://deno.com/blog/roll-your-own-javascript-runtime)

// Adaptors for web frameworks?

___

## Resources
- [windows-rs](https://github.com/microsoft/windows-rs)
- [windows-rs docs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/LibraryLoader/fn.GetModuleHandleA.html)
- [learn win32](https://learn.microsoft.com/en-us/windows/win32/learnwin32/)
- windows-rs [samples](https://github.com/microsoft/windows-rs/blob/master/crates/samples/readme.md)
- CSS Parsers:
  - [Sample CSS Parser](https://github.com/servo/servo/blob/master/components/style/stylesheets/rule_parser.rs)
  - [Alchemy Parser](https://github.com/ryanmcgrath/alchemy/blob/trunk/styles/src/styles_parser.rs)

## Insperation
- [Alchemy](https://github.com/ryanmcgrath/alchemy)
  - [Example](https://github.com/ryanmcgrath/alchemy/blob/trunk/examples/layout/src/main.rs)
  - [Cocoa](https://github.com/ryanmcgrath/alchemy/tree/trunk/cocoa)

## References
- [c_void and casting](https://stackoverflow.com/questions/24191249/working-with-c-void-in-an-ffi)
- [dcomp example](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/dcomp/src/main.rs)

## Win32 References:
- [Window Styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles)

<!-- Footer Badges --!>

<br>
<div align="center">
  <img src="assets/badges/made_with_rust.svg" alt="Made with rust"/>
  <img src="assets/badges/built_with_love.svg" alt="Built with love"/>
</div>

<!-- End Footer -->
