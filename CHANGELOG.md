# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.1 (OrbTk next) wip

### 0.3.1-alpha5 (wip)

### 0.3.1-alpha4

* Switch structure to `Workspace` organized crates:
  - removed crates dir
  - moved localization code to orbtk_core crate
  - moved theming code to orbtk_core crate
  - renamed orbtk_api to orbtk_core
  - renamed render crate to orbtk_tinyskia
  - renamed shell crate to orbtk_orbclient
  - created toplevel workspace Crago.toml
* `tiny-skia` is integrated as 2D render engine
* New `PasswordBox` widget
* New `Pager`navigation widget
* New `Clipboard` service
* New `MasterDetail` navigation widget
* Improved `TextBehavior` event handling
  - Copy Ctrl+C, Paste Ctrl+V, Cut Ctrl+X
* Remove `glupath` backend
* Refactor `on_changed` callback, add key parameter
* Access RawWindowHandle from Context
* Adopt Themes `colors selection` via CSS-like functions
* Create `Color` from HSV and HSL values
* Create a `Color` by its CSS name
* Gradient coordinates become relative to the path
* Add `gradients` defined by angle
* Introduce optional `gradients` selection in themes
* Improve `KeyboardHandler` to mark text
  - Shift + Left | Shift + Right
* Improve `MouseHandler` to select text via mouse (experimental)
* Select all character on text input by mouse double click
* Replace String16 with std::string::String
* Raise `on_changed` callback also on shared widgets
* Introduce Localization
* Custom theme names for types
  - Brush, String, Thickness, f32, f64
* Add `focus_state` property to Window
* Add `theme` property to Window
* Rename `lost_focus_on_activation` to `lose_focus_on_activation`
* Fix `Grid layout` for "auto" width
* Add thread safe `EventAdapter`
* Add thread safe `MessageAdapter` and `MessageReader`
* Add `load_async` and `save_async` to `Settings` service
* Add `TextInputEvent`
* Add `EnterEvent` and `LeaveEvent`
* Add `hover` property and selector state to all widgets
* Add `on_enter` and `on_leave` callback to `MouseHandler`
* Refactor theming (cleanup default theme, introduce state order)
* Fix theming related bugs
* Register fonts on `Theme` struct
* Update OrbTks default themes
* Add redox theme
* Add `Brush` handling gradients (respecting angles, directions, color codes)
* Add fluent design inspired theme (dark and light)
* Add fonts and icon sets specific to fluent design (Microsoft Segoe look)
* Add `message` example
* Remove widgets example
* Replaces Columns and Rows structs with Blocks
* Set grid columns and rows also as string
* Refactor grid layout code
* Add `message-handler` example
* Support features flag in `showcase` example
* Add new popup code (version 2: new placement and offset properties)
* Add `bundled` feature
* Rename `resizeable` to `resizable`
* Fix thumb position not updating when window is resized
* Upgrade cargo definitions to use rust edition 2021

### 0.3.1-alpha3

* Dynamic theme switch
* Add all material font icons as resource
* Replaces css-engine with custom Rust/Ron based theming
* Add widget access helpers for states
* API update check deprecated methods an replace to new ones
* Performance improvements
* Change state update order from tree order to incoming changes order
* NumericBox widget
* Update caret position on TextBox by mouse click
* Text input support for ', /, \, [, ], {, }
* Multiple window support (experimental)
* Pathfinder / Glutin backend (experimental)
* ProgressBar widget
* Measure distance between two Points
* Improve: Mouse event arguments
* Fix: Crash when a child widget is removed
* New `TabWidget` widget
* Add `on_changed` property change callback to all widgets
* Linking to OrbTk book

### 0.3.1-alpha2

* ComboBox / ComboboxItem widget
* Slider widget
* Popup widget
* Overlay layer
* Service registry for states
* Settings service (serialize / deserialize data)
* Direct access of states in callbacks
* Impl RawWindowHandle for Context (wip)
* Sent requests to window shell
* Layout fixes and stack layout example
* Many web fixes
* State cleanup method
* Refactor setting of styling selectors
* TextBox select all (Ctrl + a)
* Text input support for !, @, #
* Borderless window

### 0.3.1-alpha1

* api crate: base api elements of OrbTk e.g. widget and application parts
* css-engine crate: parse and read values from a css file
* proc_macros crate: procedural helper macros
* render crate: cross platform 2D/3D render library
* shell crate: cross platform window and event handling
* theme crate: OrbTks default theme (light and dark)
* tree crate: tree structure based on DCES
* utils crate: helper structs and traits
* widgets crate: base widget library
* Button widget
* Canvas widget
* CheckBox widget
* Container widget
* Cursor widget
* FontIconBlock widget
* Image widget
* Items widget
* ListView widget
* ScrollBar widget
* ScrollIndicator widget
* ScrollViewer widget
* Stack widget
* Switch widget
* TextBlock widget
* TextBox widget
* ToggleButton widget
* Window widget
