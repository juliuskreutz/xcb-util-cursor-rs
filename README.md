# xcb-util-cursor

**Documentation**: https://docs.rs/xcb-util-cursor

xcb-util-cursor is a safe rust interface for [libxcb-cursor](https://gitlab.freedesktop.org/xorg/lib/libxcb-cursor). It depends on [rust-xcb](https://crates.io/crates/xcb) and uses their types.

```toml
# Cargo.toml
[dependencies]
xcb = "1.2.0"
xcb-util-cursor = "0.3.0"
```

```rust
use xcb_util_cursor::{Cursor, CursorContext};

let (connection, _) = xcb::Connection::connect(None).unwrap();
let setup = connection.get_setup();
let screen = setup.roots().next().unwrap();

let cursor_context = CursorContext::new(&connection, screen).unwrap();

let left_ptr = cursor_context.load_cursor(Cursor::LeftPtr);
```
