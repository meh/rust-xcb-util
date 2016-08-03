#![allow(non_camel_case_types)]

extern crate xcb;
extern crate libc;

pub mod ffi;

macro_rules! define {
	(cookie $cookie:ident for $inner:ident => $reply:ident) => (
		pub struct $cookie(xcb::GetPropertyCookie,
			unsafe extern "C" fn(*mut xcb_connection_t, xcb_get_property_cookie_t, *mut $inner, *mut *mut xcb_generic_error_t) -> u8);

		impl $cookie {
			pub fn get_reply(&self) -> Result<$reply, xcb::GenericError> {
				unsafe {
					if self.0.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut reply = mem::uninitialized();
						self.1(self.0.conn, self.0.cookie, &mut reply, &mut err);

						if err.is_null() {
							Ok($reply(reply))
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut reply = mem::uninitialized();
						self.1(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

						Ok($reply(reply))
					}
				}
			}
		}
	);

	(cookie $cookie:ident with $func:ident => $reply:ident) => (
		pub struct $cookie(xcb::GetPropertyCookie);

		impl $cookie {
			pub fn get_reply(&self) -> Result<$reply, xcb::GenericError> {
				unsafe {
					if self.0.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut reply = mem::uninitialized();
						$func(self.0.conn, self.0.cookie, &mut reply, &mut err);

						if err.is_null() {
							Ok($reply(reply))
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut reply = mem::uninitialized();
						$func(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

						Ok($reply(reply))
					}
				}
			}
		}
	);

	(reply $reply:ident for $inner:ident with $wipe:ident) => (
		pub struct $reply($inner);

		impl Drop for $reply {
			fn drop(&mut self) {
				unsafe {
					$wipe(&mut self.0);
				}
			}
		}
	);
}

#[cfg(feature = "icccm")]
pub mod icccm;

#[cfg(feature = "image")]
pub mod image;
