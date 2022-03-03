# orbtk_orbclient

This crate offers a cross platform `window shell` library. It's part of
[OrbTk](https://gitlab.redox-os.org/redox-os/orbtk) - The Rust
UI-Toolkit.

[![Build and test](https://github.com/redox-os/orbtk/workflows/CI/badge.svg)](https://github.com/redox-os/orbtk/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)

## Platforms

* Redox OS
* Linux
* macOS
* Windows
* openBSD (not tested, but should work)
* Web
* Android (planned)
* iOS (planned)
* Ubuntu Touch (planned)

## Dependencies

* [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) (MIT): A macro for declaring lazily evaluated statics in Rust
* [image](https://github.com/image-rs/image) (MIT): load pixel images e.g. png
* [orbclient](https://gitlab.redox-os.org/redox-os/orbclient) (MIT): The Orbital Client Library
* [raw-window-handle](https://github.com/rust-windowing/raw-window-handle) (MIT): access to a window's platform-specific raw window handle
* [sdl2](https://www.libsdl.org) (zlib): Simple DirectMedia Layer
* [stdweb](https://github.com/koute/stdweb) (Apache 2.0, MIT): web window and events

## License

Licensed under MIT license ([LICENSE](../../LICENSE)).