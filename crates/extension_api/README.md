# The dez Rust Extension API

This crate lets you write extensions for dez in Rust.

## Extension Manifest

You'll need an `extension.toml` file at the root of your extension directory, with the following structure:

```toml
id = "my-extension"
name = "My Extension"
description = "..."
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
repository = "https://github.com/your/extension-repository"
```

## Cargo metadata

dez extensions are packaged as WebAssembly files. In your Cargo.toml, you'll
need to set your `crate-type` accordingly:

```toml
[dependencies]
dez_extension_api = "0.6.0"

[lib]
crate-type = ["cdylib"]
```

## Implementing an Extension

To define your extension, create a type that implements the `Extension` trait, and register it.

```rust
use dez_extension_api as dez;

struct MyExtension {
    // ... state
}

impl dez::Extension for MyExtension {
    // ...
}

dez::register_extension!(MyExtension);
```

## Testing your extension

To run your extension in dez as you're developing it:

- Make sure you have [Rust installed](https://www.rust-lang.org/learn/get-started)
- Have the `wasm32-wasip2` target installed (`rustup target add wasm32-wasip2`)
- Open the extensions view using the `dez: extensions` action in the command palette.
- Click the `Install Dev Extension` button in the top right
- Choose the path to your extension directory.

## Compatible dez versions

Extensions created using newer versions of the dez extension API won't be compatible with older versions of dez.

Here is the compatibility of the `dez_extension_api` with versions of dez:

| dez version | `dez_extension_api` version |
| ----------- | --------------------------- |
| `0.192.x`   | `0.0.1` - `0.6.0`           |
| `0.186.x`   | `0.0.1` - `0.5.0`           |
| `0.184.x`   | `0.0.1` - `0.4.0`           |
| `0.178.x`   | `0.0.1` - `0.3.0`           |
| `0.162.x`   | `0.0.1` - `0.2.0`           |
| `0.149.x`   | `0.0.1` - `0.1.0`           |
| `0.131.x`   | `0.0.1` - `0.0.6`           |
| `0.130.x`   | `0.0.1` - `0.0.5`           |
| `0.129.x`   | `0.0.1` - `0.0.4`           |
| `0.128.x`   | `0.0.1`                     |
