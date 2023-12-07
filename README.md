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

The goal of this project is to create building blocks for a modern web browser from scratch. The efforts of this project is also to be completely
open source and community driven.

The goals for the browser is to have it be as fast, efficient, available, and accessible as possible.

Hopefully the results of the subprojects of this larger project will be foundations for other cross-platform libraries such as game engines, desktop applicatons, parsers, web tools, and much more.

**Subprojects**
+ Cross-platform rendering abstraction to use the native OS rendering libraries.
+ Native cross-platform system libraries to create, customize, and manage windows. This would also include IO events.
+ A generic low level, easy to use CSSParser, on par with CSS L3 and allows for custom rule sets.
  + This also includes an open full browser spec css parser with utilities
  + Most likely will include parsing, compiling, and minification
+ HTML parser with DOM manipulation and functionality.
+ Javascript runtime that is fast and customizable.
+ Web protocols and interactions that are abstracted and independent.

## Contribution

Right now this project is written and maintained by a single software developer on their free time. Any and all contribution is appreciated. Additional core maintainers are welcome after major contributions.

## Modules 
+ HTML
  + DOM
  + AST
  + Hooks and customization
+ CSS Parser
  + Parser
  + Query
  + L3 Ruleset
  + Selectors
+ JS Runtime
  + WASM
  + WebGPU
  + WebGL
+ Rendering
  + Metal
  + OpenGL
  + Vulkan
  + DirectX12

- CSS Parser:
  + [cssparser (Servo)](https://docs.rs/cssparser/latest/cssparser/) with similar spec structure as Servo
- HTML Parser:
  + [html5ever (Servo)](https://github.com/servo/html5ever)
- JS Runtime:
  + [Deno](https://deno.com/blog/roll-your-own-javascript-runtime)

## Graphics API's
- [Vulkan](https://gpuopen.com/learn/hellovulkan-introductory-vulkan-sample/)
- [DirectX12](https://www.nvidia.com/en-us/geforce/technologies/dx12/)
- [Metal](https://developer.apple.com/metal/)
- [OpenGL](https://www.opengl.org/)

+ > Note: The following is how the API's will most likely be used.
  > - Vulkan (Linux)
  > - DirectX12 (Windows)
  > - Metal (Apple)
  > - OpenGL(Legacy & Mobile)

___

## Resources
- [windows-rs](https://github.com/microsoft/windows-rs)
- [windows-rs docs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/LibraryLoader/fn.GetModuleHandleA.html)
- [learn win32](https://learn.microsoft.com/en-us/windows/win32/learnwin32/)
- windows-rs [samples](https://github.com/microsoft/windows-rs/blob/master/crates/samples/readme.md)
- CSS Parsers:
  - [Sample CSS Parser](https://github.com/servo/servo/blob/master/components/style/stylesheets/rule_parser.rs)
- [Alchemy](https://github.com/ryanmcgrath/alchemy)
  - [Example](https://github.com/ryanmcgrath/alchemy/blob/trunk/examples/layout/src/main.rs)
  - [Cocoa](https://github.com/ryanmcgrath/alchemy/tree/trunk/cocoa)
- [Servo](https://github.com/servo/servo)

- [c_void and casting](https://stackoverflow.com/questions/24191249/working-with-c-void-in-an-ffi)
- [dcomp example](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/dcomp/src/main.rs)

- [Window Styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles)

<!-- Footer Badges -->

<br>
<div align="center">
  <img src="assets/badges/made_with_rust.svg" alt="Made with rust"/>
  <img src="assets/badges/built_with_love.svg" alt="Built with love"/>
</div>

<!-- End Footer -->
