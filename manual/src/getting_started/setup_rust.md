# Setup Rust

OrbTk is an UI-Toolkit written in and for the programming language [Rust](https://www.rust-lang.org/). This guide will help you to install Rust.

## Install Rust on Linux or macOS

If you are using Linux or macOS open up an terminal and copy and paste the text below and hit the enter key on your keyboard:

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Install Rust on Windows

Download and run the Rust windows installer from https://www.rust-lang.org/tools/install.

## Install Redoxer (Redox OS)

If you want build and run your Rust application on a [KVM](https://en.wikipedia.org/wiki/Kernel-based_Virtual_Machine) capable OS for Redox you can use [redoxer](https://gitlab.redox-os.org/redox-os/redoxer).

To install Redoxer you have to first install the rust toolchain. After that open up an terminal and copy and paste the text below and hit the enter key on your keyboard:

```bash
cargo install redoxer
```

To compile and run your application on Redox OS you should check the [Redox OS Book](https://doc.redox-os.org/book/getting_started/getting_started.html).