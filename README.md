# OrbTk

> OrbTk 0.4.0 is under heavy development.

The Orbital Widget Toolkit. Create scalable User Interfaces.

[![Build status](https://gitlab.com/orbtk/orbtk/badges/master/build.svg)](https://gitlab.com/orbtk/orbtk/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.2.27-orange.svg)](https://crates.io/crates/orbtk)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/orbtk)

## Features:

* Modern [Flutter](https://flutter.io/), [React](https://reactjs.org/) like API
* Uses the Entity Component System library [DCES](https://gitlab.com/orbtk/dces-rust) for widget and properties handling
* Updating instead of rebuling subtrees
* Small DSL for custom widget creation on macro base
* Flexible event system
* Widget state management
* Platform Independent: Redox OS, Linux, macOS, Window and WebAssembly

## Usage

To include OrbTk in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = "0.2.27"
```

However you also need to have the SDL2 libraries installed on your
system.  The best way to do this is documented [by the SDL2
crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).

You will also need to use the [nightly build of Rust](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) to use OrbTk. 

## Examples

You find the examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets
```

## Future features

* Style guide
* More default widgets
* More examples
* Book
* Animations
* Exchange views / widgets / screens on runtime
* Split application in modules
* Theme update

## Inspirations

* [Flutter](https://flutter.io/)
* [React](https://reactjs.org/)
* [Yew](https://github.com/DenisKolodin/yew)

