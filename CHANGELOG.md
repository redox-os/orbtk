# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.1 (OrbTk next) wip

### 0.3.1-alpha4 (wip)

* Access RawWindowHandle from Context
* Add `Brush` handling gradients (respecting angles, directions, color codes)
* Add `EnterEvent` and `LeaveEvent`
* Add `TextInputEvent`
* Add `hover` property and selector state to all widgets
* Add `load_async` and `save_async` to `Settings` service
* Add `on_enter` and `on_leave` callback to `MouseHandler`
* Add fluent design inspired theme (dark and light)
* Add focus_state property to Window
* Add fonts and icon sets specific to fluent design (Microsoft Segoe look)
* Add gradients defined by angle
* Add message example
* Add redox theme
* Add theme property to Window
* Add thread safe `EventAdapter`
* Add thread safe `MessageAdapter`, `MessageReader`
* Create `Color` from HSV and HSL values
* Create a `Color` by its CSS name
* Custom theme names for types Brush, String, Thickness, f32, f64
* Fix "auto" width of grid layout
* Fix theming related bugs
* Gradient coordinates become relative to the path
* Introduce a wip tiny-skia backend
* Localization
* MasterDetail responsive navigation widget
* Raise on_changed callback also on shared widgets
* Refactor on_changed callback, add key parameter
* Refactor theming (cleanup default theme, introduce state order)
* Register fonts on `Theme` struct
* Remove widgets example
* Replaces Columns and Rows structs with Blocks
* Set grid columns and rows also as string
* Refactor grid layout code
* Rename `lost_focus_on_activation` to `lose_focus_on_activation`
* Replace String16 with std::string::String
* Select all character on text input by mouse double click
* Select colors in themes through of CSS-like functions
* Select text my mouse (experimental)
* Temporary remove `glupath` backend
* Temporary remove `glupath` backend
* Text mark with Shift + Left | Shift + Right
* TextBehavior: Copy Ctrl+C, Paste Ctrl+V, Cut Ctrl+X
* The possibility of using gradients in themes is being introduced
* Update OrbTks default themes
* `Clipboard` service
* `MasterDetail `responsive navigation widget
* `Pager `navigation widget
* `PasswordBox` widget
* `orbraq` backend (Orbclient and raqote)

### 0.3.1-alpha3

* API update check deprecated methods an replace to new ones
* Add all material font icons as resource
* Add on_changed property change callback to all widgets
* Add widget access helpers for states
* Change state update order from tree order to incoming changes order
* Dynamic theme switch
* Fix: Crash when a child widget is removed
* Improve: Mouse event arguments
* Measure distance between two Points
* Multiple window support (experimental)
* NumericBox widget
* OrbTk book (manual) wip
* Pathfinder / Glutin backend (experimental)
* Performance improvements
* ProgressBar widget
* Replaces css-engine with custom Rust/Ron based theming
* TabWidget widget
* Text input support for ', /, \, [, ], {, }
* Update caret position on TextBox by mouse click

### 0.3.1-alpha2

* Borderless window
* ComboBox / ComboboxItem widget
* Direct access of states in callbacks
* Impl RawWindowHandle for Context (wip)
* Layout fixes and stack layout example
* Many web fixes
* Overlay layer
* Popup widget
* Refactor setting of styling selectors
* Sent requests to window shell
* Service registry for states
* Settings service (serialize / deserialize data)
* Slider widget
* State cleanup method
* Text input support for !, @, #
* TextBox select all (Ctrl + a)

### 0.3.1-alpha1

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
* api crate: base api elements of OrbTk e.g. widget and application parts
* css-engine crate: parse and read values from a css file
* proc_macros crate: procedural helper macros
* render crate: cross platform 2D/3D render library
* shell crate: cross platform window and event handling
* theme crate: OrbTks default theme (light and dark)
* tree crate: tree structure based on DCES
* utils crate: helper structs and traits
* widgets crate: base widget library
