# xcb-util-cursor

[<img alt="github" src="https://img.shields.io/badge/github-juliuskreutz/xcb--util--cursor--rs-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/juliuskreutz/xcb-util-cursor-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/xcb-util-cursor.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/xcb-util-cursor)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-xcb--util--cursor-66c2a5?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/xcb-util-cursor)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/juliuskreutz/xcb-util-cursor-rs/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/juliuskreutz/xcb-util-cursor-rs/actions?query=branch%3Amaster)
[<img alt="crates.io" src="https://img.shields.io/crates/l/xcb-util-cursor.svg?style=for-the-badge&logo=rust" height="20">](https://github.com/juliuskreutz/xcb-util-cursor-rs)

xcb-util-cursor is a safe rust interface for [libxcb-cursor](https://gitlab.freedesktop.org/xorg/lib/libxcb-cursor). It depends on [rust-xcb](https://crates.io/crates/xcb) and uses their types.

```toml
# Cargo.toml
[dependencies]
xcb = "1.6.0"
xcb-util-cursor = "0.3.5"
```

```rust
use xcb_util_cursor::{Cursor, CursorContext};

let (connection, _) = xcb::Connection::connect(None).unwrap();
let setup = connection.get_setup();
let screen = setup.roots().next().unwrap();

let cursor_context = CursorContext::new(&connection, screen).unwrap();

let left_ptr = cursor_context.load_cursor(Cursor::LeftPtr);
```
