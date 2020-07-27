# Run examples

## Native on desktop

You can find examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets --release
```

## Run Examples with cargo-node

To run the examples in a browser browser, electron or cordova app you have to install cargo-node:

```text
cargo install -f cargo-node
```

Before you can use cargo node you have to install `npm` version 6.9.0. It is included in the `Node.js` version 10.16.3. You can download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is presumed. All other dependencies of cargo node will be installed automatically.

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