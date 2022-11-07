# xcb-util-cursor

**Documentation**: https://docs.rs/xcb-util-cursor

xcb-util-cursor is a safe rust interface for [libxcb-cursor](https://gitlab.freedesktop.org/xorg/lib/libxcb-cursor). It depends on [rust-xcb](https://crates.io/crates/xcb) and uses their types.

```toml
# Cargo.toml
[dependencies]
xcb-util-cursor = "0.1.0"
```

```rust
let (connection, _) = xcb::Connection::connect(None).unwrap();
let setup = connection.get_setup();
let screen = setup.roots().next().unwrap();

let cursor_context = xcb_util_cursor::CursorContext::new(&connection, &screen).unwrap();

let left_ptr = context.load_cursor("left_ptr").unwrap();
```
