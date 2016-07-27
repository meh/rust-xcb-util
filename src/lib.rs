#![allow(non_camel_case_types)]

extern crate xcb;
extern crate libc;

pub mod ffi;

#[cfg(feature = "icccm")]
pub mod icccm;
