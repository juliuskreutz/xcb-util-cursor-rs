#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![doc(html_root_url = "https://docs.rs/xcb-util-cursor-sys/")]

pub use xcb::ffi::xcb_connection_t;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
