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

sequoia
cypress

arbol del tule
socotra dragon tree
methuselah
<!-- End Header -->

The goal of this project is to create building blocks for a modern web browser from scratch. The efforts of this project is also to be completely
open source and community driven.

Hopefully the results of the subprojects of this larger project will be foundations for other libraries such as game engines, desktop applicaton libraries
and much more.

**Subprojects**
+ Cross-platform rendering abstraction to use the best native OS rendering libraries.
+ Native cross-platform UI libraries to create customize and manage windows. This would also include IO events.
+ A generic low level, easy to use CSSParser, on par with CSS L3 and allows for custom rule sets.
+ A generic low level, easy to use HTML parser with DOM manipulation and functionality.
+ A generic low level, easy to use Javascript runtime that is fast and customizable.
+ Web protocols and interactions that are abstracted and independent.

## Modules 
+ HTML
  + DOM
    + Access
    + Manipulation
  + AST
  + Hooks and customization
+ CSS Parser
  + Parser
  + Query
  + L3 Ruleset
  + Get
+ JS Runtime
  + WASM
  + WebGPU
  + WebGL
+ Rendering
  + Metal
  + OpenGL
  + Vulkan
  + DirectX12

## Module Goals
+ HTML
  + Full DOM specs
+ CSS
  + L3+
+ JS
  + Full runtime plus customizable to add custom hooks.

## References
- CSS Parser:
  + [cssparser (Servo)](https://docs.rs/cssparser/latest/cssparser/) with similar spec structure as Servo
- HTML Parser:
  + [html5ever (Servo)](https://github.com/servo/html5ever)
- JS Runtime:
  + [Deno](https://deno.com/blog/roll-your-own-javascript-runtime)

// Adaptors for web frameworks?

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

## Insperation
- [Alchemy](https://github.com/ryanmcgrath/alchemy)
  - [Example](https://github.com/ryanmcgrath/alchemy/blob/trunk/examples/layout/src/main.rs)
  - [Cocoa](https://github.com/ryanmcgrath/alchemy/tree/trunk/cocoa)
- [Servo](https://github.com/servo/servo)

## References
- [c_void and casting](https://stackoverflow.com/questions/24191249/working-with-c-void-in-an-ffi)
- [dcomp example](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/dcomp/src/main.rs)

## Win32 References:
- [Window Styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles)

<!-- Footer Badges -->

<br>
<div align="center">
  <img src="assets/badges/made_with_rust.svg" alt="Made with rust"/>
  <img src="assets/badges/built_with_love.svg" alt="Built with love"/>
</div>

<!-- End Footer -->
