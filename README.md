<img alt="OrbTk" width="380" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/logos/orbtk/logo_dark.png">

[![Build status](https://gitlab.redox-os.org/redox-os/orbtk/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbtk/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.2.27-orange.svg)](https://crates.io/crates/orbtk)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/orbtk)

> OrbTk 0.3.0 is under heavy development and it's not compatible to earlier releases.

The Orbital Widget Toolkit is a multi platform (G)UI toolkit for building scalable user interfaces with the programming language Rust. It's based
on the [Entity Component System Pattern](https://en.wikipedia.org/wiki/Entity%E2%80%93component%E2%80%93system) and provides a functional-reactive like API. 

The main goals of OrbTk are fast performance, easy to use and cross platform.

<img alt="Calculator" width="350" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Features:

* Modern lightweight API
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
orbtk = { git = "https://gitlab.redox-os.org/redox-os/orbtk.git" }
```

However you also need to have the SDL2 libraries installed on your
system.  The best way to do this is documented [by the SDL2
crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).

## Use OrbTk with cairoe.

* With Ubuntu, please to type ```sudo apt-get install libcairo2-dev``` in your console.
* With macOS and homebrew, please to type ```brew install cairo``` in your console.
* With macOS and macports, please to type ```sudo port install cairo``` in your console.

## Minimal Example

```rust
use orbtk::prelude::*;

fn main() {
      Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::create().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
```

## Additional Examples

You find the examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets --release
```

## Additional Examples on Web

To run the examples on a browser you have to install 

```text
cargo install -f cargo-web
```

### Run

You can start the widgets example by executing the following command:

* Compile to [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) using Rust's native WebAssembly backend:

```text
cargo web start --target=wasm32-unknown-unknown --auto-reload --example widgets
```

* Compile to [asm.js](https://en.wikipedia.org/wiki/Asm.js) using Emscripten:

```text
$ cargo web start --target=asmjs-unknown-emscripten --auto-reload --example widgets
```

* Compile to WebAssembly using Emscripten:

```text
$ cargo web start --target=wasm32-unknown-emscripten --auto-reload --example widgets
```

## Run examples with Glutin and Pathfinder

OrbTk includes a preview with [Glutin](https://github.com/rust-windowing/glutin) and [Pathfinder](https://github.com/servo/pathfinder). To start the *preview* mode you have to use the feature *preview*.

By problems running OrbTk with cairo on Windows you should try the *preview*.

Pathfinder is currently not available for the web.

```text
cargo run --example widgets --release --features preview
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
* Support for Android, iOS, Ubuntu Touch and WebAssembly
* Vulkan / OpenGL Support 

## Sub Crates

* api: base api elements of OrbTk e.g. widget and application parts
* css-engine: parse and read values from a css file
* shell: cross platform window and event handling
* theme: OrbTk's default theme (light and dark)
* tree: Tree structure based on DCES
* utils: Helper structs and traits
* widgets: Base widget library

## Dependencies

* [DCES](https://gitlab.redox-os.org/redox-os/dces-rust): Entity Component System
* [rust-cssparser](https://github.com/servo/rust-cssparser): CSS parsing

## Inspirations

* [Flutter](https://flutter.io/)
* [React](https://reactjs.org/)
* [Yew](https://github.com/DenisKolodin/yew)

