//! Safe [libxcb-cursor](https://gitlab.freedesktop.org/xorg/lib/libxcb-cursor) bindings for rust.
//! You will need [xcb](https://crates.io/crates/xcb).
//!
//! # Example
//!
//! ```
//! use xcb_util_cursor::{Cursor, CursorContext};
//!
//! let (connection, _) = xcb::Connection::connect(None).unwrap();
//! let setup = connection.get_setup();
//! let screen = setup.roots().next().unwrap();
//!
//! let cursor_context = CursorContext::new(&connection, screen).unwrap();
//!
//! let left_ptr = cursor_context.load_cursor(Cursor::LeftPtr);
//! ```

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/xcb-util-cursor/")]

use std::{ffi::CString, fmt, marker, ptr};

use xcb::{Xid, XidNew, x};
use xcb_util_cursor_sys as ffi;

/// This enum provides all possible cursors. [reference](https://www.oreilly.com/library/view/x-window-system/9780937175149/ChapterD.html)
pub enum Cursor {
    /// X_cursor
    XCursor,
    /// arrow
    Arrow,
    /// based_arrow_down
    BaseArrowDown,
    /// based_arrow_up
    BasedArrowUp,
    /// boat
    Boat,
    /// bogosity
    Bogosity,
    /// bottom_left_corner
    BottomLeftCorner,
    /// bottom_right_corner
    BottomRightCorner,
    /// bottom_side
    BottomSide,
    /// bottom_tee
    BottomTee,
    /// box_spiral
    BoxSpiral,
    /// center_ptr
    CenterPtr,
    /// circle
    Circle,
    /// clock
    Clock,
    /// coffee_mug
    CoffeeMug,
    /// cross
    Cross,
    /// cross_reverse
    CrossReverse,
    /// crosshair
    Crosshair,
    /// diamond_cross
    DiamongCross,
    /// dot
    Dot,
    /// dotbox
    Dotbox,
    /// double_arrow
    DoubleArrow,
    /// draft_large
    DraftLarge,
    /// draft_small
    DrawftSmall,
    /// draped_box
    DrapedBox,
    /// exchange
    Exchange,
    /// fleur
    Fleur,
    /// gobbler
    Gobbler,
    /// gumby
    Gumby,
    /// hand1
    Hand1,
    /// hand2
    Hand2,
    /// heart
    Heart,
    /// icon
    Icon,
    /// iron_cross
    IronCross,
    /// left_ptr
    LeftPtr,
    /// left_side
    LeftSide,
    /// left_tee
    LeftTee,
    /// leftbutton
    Leftbutton,
    /// ll_angle
    LlAngle,
    /// lr_angle
    LrAngle,
    /// man
    Man,
    /// middlebutton
    Middlebutton,
    /// mouse
    Mouse,
    /// pencil
    Pencil,
    /// pirate
    Pirate,
    /// plus
    Plus,
    /// question_arrow
    QuestionArrow,
    /// right_ptr
    RightPtr,
    /// right_side
    RightSide,
    /// right_tee
    RightTee,
    /// rightbutton
    Rightbutton,
    /// rtl_logo
    RtlLogo,
    /// sailboat
    Sailboat,
    /// sb_down_arrow
    SbDownArrow,
    /// sb_h_double_arrow
    SbHDoubleArrow,
    /// sb_left_arrow
    SbLeftArrow,
    /// sb_right_arrow
    SbRightArrow,
    /// sb_up_arrow
    SbUpArrow,
    /// sb_v_double_arrow
    SbVDoubleArrow,
    /// shuttle
    Shuttle,
    /// sizing
    Sizing,
    /// spider
    Spider,
    /// spraycan
    Spraycan,
    /// star
    Star,
    /// target
    Target,
    /// tcross
    Tcross,
    /// top_left_arrow
    TopLeftArrow,
    /// top_left_corner
    TopLeftCorner,
    /// top_right_corner
    TopRightCorner,
    /// top_side
    TopSide,
    /// top_tee
    TopTee,
    /// trek
    Trek,
    /// ul_angle
    UlAngle,
    /// umbrella
    Umbrella,
    /// ur_angle
    UrAngle,
    /// watch
    Watch,
    /// xterm
    Xterm,
    /// Custom cursor name
    Custom(String),
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cursor::XCursor => "X_cursor",
            Cursor::Arrow => "arrow",
            Cursor::BaseArrowDown => "based_arrow_down",
            Cursor::BasedArrowUp => "based_arrow_up",
            Cursor::Boat => "boat",
            Cursor::Bogosity => "bogosity",
            Cursor::BottomLeftCorner => "bottom_left_corner",
            Cursor::BottomRightCorner => "bottom_right_corner",
            Cursor::BottomSide => "bottom_side",
            Cursor::BottomTee => "bottom_tee",
            Cursor::BoxSpiral => "box_spiral",
            Cursor::CenterPtr => "center_ptr",
            Cursor::Circle => "circle",
            Cursor::Clock => "clock",
            Cursor::CoffeeMug => "coffee_mug",
            Cursor::Cross => "cross",
            Cursor::CrossReverse => "cross_reverse",
            Cursor::Crosshair => "crosshair",
            Cursor::DiamongCross => "diamond_cross",
            Cursor::Dot => "dot",
            Cursor::Dotbox => "dotbox",
            Cursor::DoubleArrow => "double_arrow",
            Cursor::DraftLarge => "draft_large",
            Cursor::DrawftSmall => "draft_small",
            Cursor::DrapedBox => "draped_box",
            Cursor::Exchange => "exchange",
            Cursor::Fleur => "fleur",
            Cursor::Gobbler => "gobbler",
            Cursor::Gumby => "gumby",
            Cursor::Hand1 => "hand1",
            Cursor::Hand2 => "hand2",
            Cursor::Heart => "heart",
            Cursor::Icon => "icon",
            Cursor::IronCross => "iron_cross",
            Cursor::LeftPtr => "left_ptr",
            Cursor::LeftSide => "left_side",
            Cursor::LeftTee => "left_tee",
            Cursor::Leftbutton => "leftbutton",
            Cursor::LlAngle => "ll_angle",
            Cursor::LrAngle => "lr_angle",
            Cursor::Man => "man",
            Cursor::Middlebutton => "middlebutton",
            Cursor::Mouse => "mouse",
            Cursor::Pencil => "pencil",
            Cursor::Pirate => "pirate",
            Cursor::Plus => "plus",
            Cursor::QuestionArrow => "question_arrow",
            Cursor::RightPtr => "right_ptr",
            Cursor::RightSide => "right_side",
            Cursor::RightTee => "right_tee",
            Cursor::Rightbutton => "rightbutton",
            Cursor::RtlLogo => "rtl_logo",
            Cursor::Sailboat => "sailboat",
            Cursor::SbDownArrow => "sb_down_arrow",
            Cursor::SbHDoubleArrow => "sb_h_double_arrow",
            Cursor::SbLeftArrow => "sb_left_arrow",
            Cursor::SbRightArrow => "sb_right_arrow",
            Cursor::SbUpArrow => "sb_up_arrow",
            Cursor::SbVDoubleArrow => "sb_v_double_arrow",
            Cursor::Shuttle => "shuttle",
            Cursor::Sizing => "sizing",
            Cursor::Spider => "spider",
            Cursor::Spraycan => "spraycan",
            Cursor::Star => "star",
            Cursor::Target => "target",
            Cursor::Tcross => "tcross",
            Cursor::TopLeftArrow => "top_left_arrow",
            Cursor::TopLeftCorner => "top_left_corner",
            Cursor::TopRightCorner => "top_right_corner",
            Cursor::TopSide => "top_side",
            Cursor::TopTee => "top_tee",
            Cursor::Trek => "trek",
            Cursor::UlAngle => "ul_angle",
            Cursor::Umbrella => "umbrella",
            Cursor::UrAngle => "ur_angle",
            Cursor::Watch => "watch",
            Cursor::Xterm => "xterm",
            Cursor::Custom(s) => s,
        };

        write!(f, "{s}")
    }
}

/// Wrapper sctruct for xcb_cursor_context_t that handles creation and freeing.
pub struct CursorContext<'a> {
    ctx: *mut ffi::xcb_cursor_context_t,
    phantom: marker::PhantomData<&'a xcb::Connection>,
}

impl<'a> CursorContext<'a> {
    /// Create a new cursor context.
    pub fn new(connection: &'a xcb::Connection, screen: &x::Screen) -> Option<Self> {
        let mut screen = ffi::xcb_screen_t {
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
            ffi::xcb_cursor_context_new(connection.get_raw_conn(), &mut screen, &mut ctx)
        };

        if res != 0 {
            None
        } else {
            Some(Self {
                ctx,
                phantom: marker::PhantomData,
            })
        }
    }

    /// Loads a cursor. Returns CURSOR_NONE on error.
    pub fn load_cursor(&self, cursor: Cursor) -> x::Cursor {
        let c_str = CString::new(cursor.to_string()).unwrap();

        unsafe {
            let cursor = ffi::xcb_cursor_load_cursor(self.ctx, c_str.as_ptr());
            x::Cursor::new(cursor)
        }
    }
}

impl<'a> Drop for CursorContext<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::xcb_cursor_context_free(self.ctx);
        }
    }
}
