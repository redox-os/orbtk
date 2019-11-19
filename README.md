<img alt="OrbTk" width="380" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/logos/orbtk/logo_dark.png">

[![Build status](https://gitlab.redox-os.org/redox-os/orbtk/badges/develop/build.svg)](https://gitlab.redox-os.org/redox-os/orbtk/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.3.1alpha1-orange.svg)](https://crates.io/crates/orbtk/0.3.1-alpha1)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/orbtk)

> OrbTk 0.3.* is under heavy development and is not compatible with earlier releases.

The Orbital Widget Toolkit is a multi platform (G)UI toolkit for building scalable user interfaces with the programming language Rust. It's based
on the [Entity Component System Pattern](https://en.wikipedia.org/wiki/Entity_system) and provides a functional-reactive like API.

The main goals of OrbTk are speed, ease of use, and being cross platform.

<img alt="Calculator" width="350" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Features:

* Modern lightweight API
* Uses the Entity Component System library [DCES](https://gitlab.redox-os.org/redox-os/dces-rust) for widget and properties handling
* Updating instead of rebuilding sub-trees
* Flexible event system
* Widget state management
* Cross platform: Redox OS, Linux, macOS, Windows
* CSS theming

## Platforms

* Redox OS (native | cargo-node)
* Linux (native | cargo-node)
* macOS (native | cargo-node)
* Windows (native | cargo-node)
* openBSD (not tested, but should work)
* Web (cargo-node)
* Android (native planned after 0.3 | cargo-node)
* iOS (native planned after 0.3 | cargo-node planned after 0.3)
* Ubuntu Touch (native planned  after 0.3 | cargo-node planned for 0.3)

## Usage

To include OrbTk in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = "0.3.1-alpha1"
```

To use the latest development version of OrbTk, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = { git = "https://gitlab.redox-os.org/redox-os/orbtk.git", branch = "develop" }
```

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

## Run Examples

You can find examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets --release
```

## Run Examples with cargo-node

To run the examples on as browser, electron or cordova app you have to install

```text
cargo install -f cargo-node
```

Before you could use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You could download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatic.

### Start examples

You can start the widgets example by executing the following command:

* Run as browser app:

```text
cargo node run --target browser --example widgets
```

* Run as electron app:

```text
cargo node run --target electron --example widgets
```

* Run as cordova app on android:

```text
cargo node run --target android --example widgets
```

## Build and run documentation

You can build and run the latest documentation by executing the following command:

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
* 3D support

## Sub Crates

* [api](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/api): base api elements of OrbTk e.g. widget and application parts
* [css-engine](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/css-engine): parse and read values from a css file
* [proc-macros](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/proc-macros): procedural helper macros
* [render](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/render): cross platform 2D/3D render library
* [shell](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/api): cross platform window and event handling
* [theme](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/theme): OrbTks default theme (light and dark)
* [tree](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/tree): tree structure based on DCES
* [utils](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/utils): helper structs and traits
* [widgets](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/widgets): base widget library

## Inspirations

* [Flutter](https://flutter.io/)
* [React](https://reactjs.org/)
* [Yew](https://github.com/DenisKolodin/yew)

## Showcases

* [Space Editor](https://codeberg.org/pinhead-galaxy/space-editor): 2D Tile Map Editor compatible with OrbGame
* [OrbCalculator](https://gitlab.redox-os.org/redox-os/orbcalculator): Calculator based on OrbTk

## Contribution

If you want to help bring OrbTk further or you have feedback check our issue board https://gitlab.redox-os.org/redox-os/orbtk/-/boards/120. You could also discuss with us about OrbTk on the Redox chat https://redox-os.org/community/ (join the orbital channel).

## License

Licensed under MIT license ([LICENSE](LICENSE)).