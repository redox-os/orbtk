# orbtk
The Orbital Widget Toolkit. Compatible with Redox and SDL2.

[![Travis Build Status](https://travis-ci.org/redox-os/orbtk.svg?branch=master)](https://travis-ci.org/redox-os/orbtk)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/orbtk)](https://crates.io/crates/orbtk)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/orbtk)

## Usage

To include orbtk in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = "0.2.26"
```

However you also need to have the SDL2 libraries installed on your
system.  The best way to do this is documented [by the SDL2
crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).

You will also need to use the [nightly build of Rust](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) to use orbtk. 

## Examples

You find the examples in the `examples/` directory.

You can start the adventure example by executing the following command:

```text
cargo run --example widgets
```
