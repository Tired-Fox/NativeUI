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

## Names
  - Apple API > Braeburn
  - Linux API > Humboldt
  - Windows > Skylight

## Core
- Wrapper around objects to allow for styling and events
  - This includes methods for toggling visibility and custom drawing
  - Wrapper has handlers for events to provide interactivity
  - Constructors and destructors
- Container objects have automatic support for scrolling

## Rendering
- Containers to create tree like structure
- Layout system
- Can manipulate core elements into a layout
- renders the system and provides style updates

## Styling
- CSS at it's core using the same styles and shorthands
- Styles are passed to elements on update/draw

## Elements
  - Scroll Bar
  - Rich Edit
  - Combo Box
  - List Box
  - Edit control
  - Button
  - Text
  - List
  - Dropdown
  - Menu
  - Window

## Resources
- [windows-rs](https://github.com/microsoft/windows-rs)
- [windows-rs docs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/LibraryLoader/fn.GetModuleHandleA.html)
- [learn win32](https://learn.microsoft.com/en-us/windows/win32/learnwin32/)
- windows-rs [samples](https://github.com/microsoft/windows-rs/blob/master/crates/samples/readme.md)
- [rust-cssparser](https://github.com/servo/rust-cssparser)
  - [Sample Parser](https://github.com/servo/servo/blob/master/components/style/stylesheets/rule_parser.rs)
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
