//! Safe [libxcb-cursor](https://gitlab.freedesktop.org/xorg/lib/libxcb-cursor) bindings for rust.
//! You will need [xcb](https://crates.io/crates/xcb).
//!
//! # Example
//!
//! ```
//! let (connection, _) = xcb::Connection::connect(None).unwrap();
//! let setup = connection.get_setup();
//! let screen = setup.roots().next().unwrap();
//!
//! let cursor_context = xcb_util_cursor::CursorContext::new(&connection, &screen).unwrap();
//!
//! let left_ptr = context.load_cursor("left_ptr").unwrap();
//! ```

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/xcb-util-cursor/")]

use std::{ffi, ptr};

use anyhow::Result;
use strum::Display;
use xcb::{x, Xid, XidNew};
use xcb_util_cursor_sys as sys;

/// This enum provides all possible cursors. [reference](https://www.oreilly.com/library/view/x-window-system/9780937175149/ChapterD.html)
#[derive(Display)]
pub enum Cursor {
    /// X_cursor
    #[strum(serialize = "X_cursor")]
    XCursor,
    /// arrow
    #[strum(serialize = "arrow")]
    Arrow,
    /// based_arrow_down
    #[strum(serialize = "based_arrow_down")]
    BaseArrowDown,
    /// based_arrow_up
    #[strum(serialize = "based_arrow_up")]
    BasedArrowUp,
    /// boat
    #[strum(serialize = "boat")]
    Boat,
    /// bogosity
    #[strum(serialize = "bogosity")]
    Bogosity,
    /// bottom_left_corner
    #[strum(serialize = "bottom_left_corner")]
    BottomLeftCorner,
    /// bottom_right_corner
    #[strum(serialize = "bottom_right_corner")]
    BottomRightCorner,
    /// bottom_side
    #[strum(serialize = "bottom_side")]
    BottomSide,
    /// bottom_tee
    #[strum(serialize = "bottom_tee")]
    BottomTee,
    /// box_spiral
    #[strum(serialize = "box_spiral")]
    BoxSpiral,
    /// center_ptr
    #[strum(serialize = "center_ptr")]
    CenterPtr,
    /// circle
    #[strum(serialize = "circle")]
    Circle,
    /// clock
    #[strum(serialize = "clock")]
    Clock,
    /// coffee_mug
    #[strum(serialize = "coffee_mug")]
    CoffeeMug,
    /// cross
    #[strum(serialize = "cross")]
    Cross,
    /// cross_reverse
    #[strum(serialize = "cross_reverse")]
    CrossReverse,
    /// crosshair
    #[strum(serialize = "crosshair")]
    Crosshair,
    /// diamond_cross
    #[strum(serialize = "diamond_cross")]
    DiamongCross,
    /// dot
    #[strum(serialize = "dot")]
    Dot,
    /// dotbox
    #[strum(serialize = "dotbox")]
    Dotbox,
    /// double_arrow
    #[strum(serialize = "double_arrow")]
    DoubleArrow,
    /// draft_large
    #[strum(serialize = "draft_large")]
    DraftLarge,
    /// draft_small
    #[strum(serialize = "draft_small")]
    DrawftSmall,
    /// draped_box
    #[strum(serialize = "draped_box")]
    DrapedBox,
    /// exchange
    #[strum(serialize = "exchange")]
    Exchange,
    /// fleur
    #[strum(serialize = "fleur")]
    Fleur,
    /// gobbler
    #[strum(serialize = "gobbler")]
    Gobbler,
    /// gumby
    #[strum(serialize = "gumby")]
    Gumby,
    /// hand1
    #[strum(serialize = "hand1")]
    Hand1,
    /// hand2
    #[strum(serialize = "hand2")]
    Hand2,
    /// heart
    #[strum(serialize = "heart")]
    Heart,
    /// icon
    #[strum(serialize = "icon")]
    Icon,
    /// iron_cross
    #[strum(serialize = "iron_cross")]
    IronCross,
    /// left_ptr
    #[strum(serialize = "left_ptr")]
    LeftPtr,
    /// left_side
    #[strum(serialize = "left_side")]
    LeftSide,
    /// left_tee
    #[strum(serialize = "left_tee")]
    LeftTee,
    /// leftbutton
    #[strum(serialize = "leftbutton")]
    Leftbutton,
    /// ll_angle
    #[strum(serialize = "ll_angle")]
    LlAngle,
    /// lr_angle
    #[strum(serialize = "lr_angle")]
    LrAngle,
    /// man
    #[strum(serialize = "man")]
    Man,
    /// middlebutton
    #[strum(serialize = "middlebutton")]
    Middlebutton,
    /// mouse
    #[strum(serialize = "mouse")]
    Mouse,
    /// pencil
    #[strum(serialize = "pencil")]
    Pencil,
    /// pirate
    #[strum(serialize = "pirate")]
    Pirate,
    /// plus
    #[strum(serialize = "plus")]
    Plus,
    /// question_arrow
    #[strum(serialize = "question_arrow")]
    QuestionArrow,
    /// right_ptr
    #[strum(serialize = "right_ptr")]
    RightPtr,
    /// right_side
    #[strum(serialize = "right_side")]
    RightSide,
    /// right_tee
    #[strum(serialize = "right_tee")]
    RightTee,
    /// rightbutton
    #[strum(serialize = "rightbutton")]
    Rightbutton,
    /// rtl_logo
    #[strum(serialize = "rtl_logo")]
    RtlLogo,
    /// sailboat
    #[strum(serialize = "sailboat")]
    Sailboat,
    /// sb_down_arrow
    #[strum(serialize = "sb_down_arrow")]
    SbDownArrow,
    /// sb_h_double_arrow
    #[strum(serialize = "sb_h_double_arrow")]
    SbHDoubleArrow,
    /// sb_left_arrow
    #[strum(serialize = "sb_left_arrow")]
    SbLeftArrow,
    /// sb_right_arrow
    #[strum(serialize = "sb_right_arrow")]
    SbRightArrow,
    /// sb_up_arrow
    #[strum(serialize = "sb_up_arrow")]
    SbUpArrow,
    /// sb_v_double_arrow
    #[strum(serialize = "sb_v_double_arrow")]
    SbVDoubleArrow,
    /// shuttle
    #[strum(serialize = "shuttle")]
    Shuttle,
    /// sizing
    #[strum(serialize = "sizing")]
    Sizing,
    /// spider
    #[strum(serialize = "spider")]
    Spider,
    /// spraycan
    #[strum(serialize = "spraycan")]
    Spraycan,
    /// star
    #[strum(serialize = "star")]
    Star,
    /// target
    #[strum(serialize = "target")]
    Target,
    /// tcross
    #[strum(serialize = "tcross")]
    Tcross,
    /// top_left_arrow
    #[strum(serialize = "top_left_arrow")]
    TopLeftArrow,
    /// top_left_corner
    #[strum(serialize = "top_left_corner")]
    TopLeftCorner,
    /// top_right_corner
    #[strum(serialize = "top_right_corner")]
    TopRightCorner,
    /// top_side
    #[strum(serialize = "top_side")]
    TopSide,
    /// top_tee
    #[strum(serialize = "top_tee")]
    TopTee,
    /// trek
    #[strum(serialize = "trek")]
    Trek,
    /// ul_angle
    #[strum(serialize = "ul_angle")]
    UlAngle,
    /// umbrella
    #[strum(serialize = "umbrella")]
    Umbrella,
    /// ur_angle
    #[strum(serialize = "ur_angle")]
    UrAngle,
    /// watch
    #[strum(serialize = "watch")]
    Watch,
    /// xterm
    #[strum(serialize = "xterm")]
    Xterm,
}

/// Wrapper sctruct for xcb_cursor_context_t that handles creation and freeing.
pub struct CursorContext {
    inner: ptr::NonNull<sys::xcb_cursor_context_t>,
}

impl CursorContext {
    /// Create a new cursor context.
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

    /// Loads a cursor.
    pub fn load_cursor(&self, cursor: Cursor) -> Result<x::Cursor> {
        unsafe {
            let c_str = ffi::CString::new(cursor.to_string())?;
            let cursor = sys::xcb_cursor_load_cursor(self.inner.as_ptr(), c_str.as_ptr());

            if cursor == x::CURSOR_NONE.resource_id() {
                anyhow::bail!("Could not load {}", cursor);
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
