# Cypress System Crate

This crate includes logic for creating windows and making sys calls. It has an API that maps to the appropriate platforms
libraries. The base crate will give access to creating a windows, themed title bars, light and dark mode background colors,
window/system events, I/O events. On top of this basic modals and selector APIs are provided. The idea is that the base
crate serves to provide a base window for other libraries such as those that use graphics APIs to render to the windows.

Additionally, to begin with this crate will provide an API for systray utilities and other useful features including full
application generation using native system API calls.

## Features:

- [ ] Create a Window
  - [ ] Windows
  - [ ] Linux
  - [ ] Macos
- [ ] Modals and Selectors
  - [ ] Dialog
  - [ ] File Selector
- [ ] Events
  - [ ] Keyboard
  - [ ] Mouse
  - [ ] Actions
