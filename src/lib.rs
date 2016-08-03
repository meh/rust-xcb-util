#![allow(non_camel_case_types)]

extern crate xcb;
extern crate libc;

pub mod ffi;

#[macro_use]
pub mod util;

#[cfg(feature = "icccm")]
pub mod icccm;

#[cfg(feature = "image")]
pub mod image;
