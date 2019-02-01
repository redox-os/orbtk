<img alt="OrbTk" height="200" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/logos/orbtk/logo_dark.png">

[![Build status](https://gitlab.redox-os.org/redox-os/orbtk/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbtk/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.2.27-orange.svg)](https://crates.io/crates/orbtk)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/orbtk)

> OrbTk 0.3.0 is under heavy development and it's not compatible to earlier releases.

The Orbital Widget Toolkit is a multi platform (G)UI toolkit for building scalable user interfaces with the programming language Rust. It's based
on the [Entity Component System Pattern](https://en.wikipedia.org/wiki/Entity%E2%80%93component%E2%80%93system) and provides a functional-reactive API. 

The main goals of OrbTk are fast performance, easy to use and cross platform.

<img alt="Calculator" height="300" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Features:

* Modern [Flutter](https://flutter.io/), [React](https://reactjs.org/), [Redux](https://redux.js.org/) like API
* Uses the Entity Component System library [DCES](https://gitlab.redox-os.org/redox-os/dces-rust) for widget and properties handling
* Updating instead of rebuilding sub-trees
* Flexible event system
* Widget state management
* Cross platform: Redox OS, Linux, macOS, Windows
* CSS theming

## Usage

To include OrbTk in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = "0.2.27"
```

To use OrbTk 0.3, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = { git = https://gitlab.redox-os.org/redox-os/orbtk.git }
```

However you also need to have the SDL2 libraries installed on your
system.  The best way to do this is documented [by the SDL2
crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).

## Minimal Example

```rust
use orbtk::*;

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds(Bounds::new(100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - minimal example")
        .root(TextBlock::create().text("OrbTk").into())
        .debug_flag(false)
        .build();
    application.run();
}
```

## Additional Examples

You find the examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets --release
```

## Build and run documentation

You can build and run the latest documentation y executing the following command:

```text
cargo doc --no-deps --open
```

## Planned features

* Style guide
* More default widgets
* More examples
* Book
* Animations
* Exchange views / widgets / screens on runtime
* Split application in modules
* Theme update
* Support for Android, iOS and WebAssembly
* Vulkan / OpenGL Support 

## Dependencies

* [OrbClient](https://gitlab.redox-os.org/redox-os/orbclient): window creation, drawing, window events
* [OrbFont](https://gitlab.redox-os.org/redox-os/orbfont): font rendering
* [OrbImage](https://gitlab.redox-os.org/redox-os/orbimage/tree/master/src): image loading
* [DCES](https://gitlab.redox-os.org/redox-os/dces-rust): Entity Component System
* [rust-cssparser](https://github.com/servo/rust-cssparser): CSS parsing

## Inspirations

* [Flutter](https://flutter.io/)
* [React](https://reactjs.org/)
* [Yew](https://github.com/DenisKolodin/yew)

