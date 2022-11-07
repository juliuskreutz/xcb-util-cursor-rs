use std::{ffi, ptr};

use anyhow::Result;
use xcb::{x, Xid, XidNew};
use xcb_util_cursor_sys as sys;

struct CursorContext {
    inner: ptr::NonNull<sys::xcb_cursor_context_t>,
}

impl CursorContext {
    pub fn new(connection: &xcb::Connection, screen: &x::Screen) -> Result<Self> {
        let mut screen = sys::xcb_screen_t {
            root: screen.root().resource_id(),
            default_colormap: screen.default_colormap().resource_id(),
            white_pixel: screen.white_pixel(),
            black_pixel: screen.black_pixel(),
            current_input_masks: screen.current_input_masks().bits(),
            width_in_pixels: screen.width_in_pixels(),
            height_in_pixels: screen.height_in_pixels(),
            width_in_millimeters: screen.width_in_millimeters(),
            height_in_millimeters: screen.height_in_millimeters(),
            min_installed_maps: screen.min_installed_maps(),
            max_installed_maps: screen.max_installed_maps(),
            root_visual: screen.root_visual(),
            backing_stores: screen.backing_stores() as u8,
            save_unders: screen.save_unders() as u8,
            root_depth: screen.root_depth(),
            allowed_depths_len: screen.allowed_depths().count() as u8,
        };

        let mut ctx = ptr::null_mut();

        let res = unsafe {
            sys::xcb_cursor_context_new(
                connection.get_raw_conn() as *mut sys::xcb_connection_t,
                &mut screen,
                &mut ctx,
            )
        };

        let Some(ctx) = ptr::NonNull::new(ctx) else {
            anyhow::bail!(res);
        };

        Ok(Self { inner: ctx })
    }

    pub fn load_cursor(&self, name: &str) -> Result<x::Cursor> {
        unsafe {
            let c_str = ffi::CString::new(name)?;
            let cursor = sys::xcb_cursor_load_cursor(self.inner.as_ptr(), c_str.as_ptr());

            if cursor == x::CURSOR_NONE.resource_id() {
                anyhow::bail!("Could not load {}", name);
            }

            Ok(x::Cursor::new(cursor))
        }
    }
}

impl Drop for CursorContext {
    fn drop(&mut self) {
        unsafe {
            sys::xcb_cursor_context_free(self.inner.as_ptr());
        }
    }
}
