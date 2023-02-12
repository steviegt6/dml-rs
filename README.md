# dml-rs

> `display_mouse_lock` in rust.

Free, open-source reimplementation of `display_mouse_lock` ([Native Mouse Lock](https://marketplace.yoyogames.com/assets/9857/native-mouse-lock)) in Rust. Written because I felt like it. And because it was easy.

# Notes

- Considerably smaller binary - 3 kilobytes as opposed to 90 kilobytes.
- Lack `display_mouse_bounds` support.
  - Low priority as it is not used in [HoloCure](https://kay-yu.itch.io/holocure), which is what I wrote this for.
- No \*nix support.
  - AFAIK, Native Mouse Lock is Windows-only as well, but I'd like to support \*nix at some point. Feel free to PR.

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

1. Download the latest release from the [releases page](https://github.com/steviegt6/dml-rs/releases).
2. Copy the `dml_rs.dll` file to your game's directory.
3. Rename `dml_rs.dll` to `display_mouse_lock_x64.dll` or whatever the name of the original DLL is.

## For In-Development Games

1. I do not know.
2. I do not know.
3. I do not know.
