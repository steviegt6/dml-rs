# dml-rs

> `display_mouse_lock` in rust.

Free, open-source reimplementation of `display_mouse_lock` ([Native Mouse Lock](https://marketplace.yoyogames.com/assets/9857/native-mouse-lock)) in Rust. Written because I felt like it. And because it was easy.

# Building

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Compiling

```bash
git clone https://github.com/steviegt6/dml-rs.git
cd dml-rs
cargo build --release
```

# Usage

## For In-Production Games

1. Download the latest release from the [releases page]().
2. Copy the `libdml.dll` file to your game's directory.
3. Rename `libdml.dll` to `display_mouse_lock_x64.dll` or whatever the name of the original DLL is.

## For In-Development Games

1. I do not know.
2. I do not know.
3. I do not know.
