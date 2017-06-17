macro_rules! define {
	(cookie $cookie:ident for $inner:ident => $reply:ident) => (
		pub struct $cookie<'a>(xcb::GetPropertyCookie<'a>,
			unsafe extern "C" fn(*mut xcb_connection_t, xcb_get_property_cookie_t, *mut $inner, *mut *mut xcb_generic_error_t) -> u8);

		#[cfg(feature = "thread")]
		unsafe impl<'a> Send for $cookie<'a> { }
		#[cfg(feature = "thread")]
		unsafe impl<'a> Sync for $cookie<'a> { }

		impl<'a> $cookie<'a> {
			pub fn get_reply(&self) -> Result<$reply, xcb::GenericError> {
				unsafe {
					if self.0.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut reply = mem::zeroed();
						self.1(self.0.conn.get_raw_conn(), self.0.cookie, &mut reply, &mut err);

						if err.is_null() {
							Ok($reply(reply))
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut reply = mem::zeroed();
						self.1(self.0.conn.get_raw_conn(), self.0.cookie, &mut reply, ptr::null_mut());

						Ok($reply(reply))
					}
				}
			}
		}
	);

	(cookie $cookie:ident with $func:ident => $reply:ident) => (
		pub struct $cookie<'a>(xcb::GetPropertyCookie<'a>);

		#[cfg(feature = "thread")]
		unsafe impl<'a> Send for $cookie<'a> { }
		#[cfg(feature = "thread")]
		unsafe impl<'a> Sync for $cookie<'a> { }

		impl<'a> $cookie<'a> {
			pub fn get_reply(&self) -> Result<$reply, xcb::GenericError> {
				unsafe {
					if self.0.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut reply = mem::zeroed();
						$func(self.0.conn.get_raw_conn(), self.0.cookie, &mut reply, &mut err);

						if err.is_null() {
							Ok($reply(reply))
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut reply = mem::zeroed();
						$func(self.0.conn.get_raw_conn(), self.0.cookie, &mut reply, ptr::null_mut());

						Ok($reply(reply))
					}
				}
			}
		}
	);

	(cookie $cookie:ident through $conn:ident with $func:ident => $reply:ident) => (
		pub struct $cookie<'a> {
			conn:    &'a $conn,
			cookie:  xcb_get_property_cookie_t,
			checked: bool,
		}

		#[cfg(feature = "thread")]
		unsafe impl<'a> Send for $cookie<'a> { }
		#[cfg(feature = "thread")]
		unsafe impl<'a> Sync for $cookie<'a> { }

		impl<'a> $cookie<'a> {
			pub fn get_reply(&self) -> Result<$reply, xcb::GenericError> {
				unsafe {
					if self.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut reply = mem::zeroed();
						let     res = $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

						if err.is_null() && res != 0 {
							Ok($reply(reply))
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut reply = mem::zeroed();
						$func(self.conn.get_raw_conn(), self.cookie, &mut reply, ptr::null_mut());

						Ok($reply(reply))
					}
				}
			}
		}
	);

	(cookie $cookie:ident through $conn:ident with $func:ident as ($first:path, $second:path)) => (
		pub struct $cookie<'a> {
			conn:    &'a $conn,
			cookie:  xcb_get_property_cookie_t,
			checked: bool,
		}

		#[cfg(feature = "thread")]
		unsafe impl<'a> Send for $cookie<'a> { }
		#[cfg(feature = "thread")]
		unsafe impl<'a> Sync for $cookie<'a> { }

		impl<'a> $cookie<'a> {
			pub fn get_reply(&self) -> Result<($first, $second), xcb::GenericError> {
				unsafe {
					if self.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut first = mem::zeroed();
						let mut second = mem::zeroed();
						let     res = $func(self.conn.get_raw_conn(), self.cookie, &mut first, &mut second, &mut err);

						if err.is_null() && res != 0 {
							Ok((first, second))
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut first = mem::zeroed();
						let mut second = mem::zeroed();
						$func(self.conn.get_raw_conn(), self.cookie, &mut first, &mut second, ptr::null_mut());

						Ok((first, second))
					}
				}
			}
		}
	);

	(cookie $cookie:ident through $conn:ident with $func:ident as $reply:path) => (
		pub struct $cookie<'a> {
			conn:    &'a $conn,
			cookie:  xcb_get_property_cookie_t,
			checked: bool,
		}

		#[cfg(feature = "thread")]
		unsafe impl<'a> Send for $cookie<'a> { }
		#[cfg(feature = "thread")]
		unsafe impl<'a> Sync for $cookie<'a> { }

		impl<'a> $cookie<'a> {
			pub fn get_reply(&self) -> Result<$reply, xcb::GenericError> {
				unsafe {
					if self.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut reply = mem::zeroed();
						let     res = $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

						if err.is_null() && res != 0 {
							Ok(reply)
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut reply = mem::zeroed();
						$func(self.conn.get_raw_conn(), self.cookie, &mut reply, ptr::null_mut());

						Ok(reply)
					}
				}
			}
		}
	);

	(cookie $cookie:ident($inner:path) through $conn:ident with $func:ident as $reply:path) => (
		pub struct $cookie<'a> {
			conn:    &'a $conn,
			cookie:  $inner,
			checked: bool,
		}

		#[cfg(feature = "thread")]
		unsafe impl<'a> Send for $cookie<'a> { }
		#[cfg(feature = "thread")]
		unsafe impl<'a> Sync for $cookie<'a> { }

		impl<'a> $cookie<'a> {
			pub fn get_reply(&self) -> Result<$reply, xcb::GenericError> {
				unsafe {
					if self.checked {
						let mut err: *mut xcb_generic_error_t = ptr::null_mut();
						let mut reply = mem::zeroed();
						let     res = $func(self.conn.get_raw_conn(), self.cookie, &mut reply, &mut err);

						if err.is_null() && res != 0 {
							Ok(reply)
						}
						else {
							Err(xcb::GenericError { ptr: err })
						}
					}
					else {
						let mut reply = mem::zeroed();
						$func(self.conn.get_raw_conn(), self.cookie, &mut reply, ptr::null_mut());

						Ok(reply)
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

	(reply $reply:ident for $inner:ident) => (
		pub struct $reply($inner);
	);
}

macro_rules! void {
	(checked -> $conn:expr, $cookie:expr) => (unsafe {
		xcb::VoidCookie {
			cookie:  $cookie,
			conn:    $conn,
			checked: true,
		}
	});

	(unchecked -> $conn:expr, $cookie:expr) => (unsafe {
		xcb::VoidCookie {
			cookie:  $cookie,
			conn:    $conn,
			checked: true,
		}
	});
}

macro_rules! property {
	(checked $name:ident -> $conn:expr, $cookie:expr) => (unsafe {
		$name {
			conn:    $conn,
			cookie:  $cookie,
			checked: true,
		}
	});

	(unchecked $name:ident -> $conn:expr, $cookie:expr) => (unsafe {
		$name {
			conn:    $conn,
			cookie:  $cookie,
			checked: false,
		}
	});
}

pub mod utf8 {
	use std::str;
	use std::slice;
	use libc::c_char;

	pub fn into<'a>(data: *const c_char, length: u32) -> Vec<&'a str> {
		if length == 0 {
			return Vec::new();
		}
		unsafe {
			let mut strs = str::from_utf8_unchecked(slice::from_raw_parts(data as *mut u8, length as usize))
				.split('\0')
				.collect::<Vec<_>>();
			// data is sometimes null-terminated and sometimes not. If there is
			// a null terminator, then our call to .split() will result in an
			// extra empty-string element at the end.
			if let Some(&"") = strs.last() {
				strs.pop();
			}
			strs
		}
	}

	pub fn from<'a, T: IntoIterator<Item=&'a str>>(data: T) -> Vec<u8> {
		let mut result = String::new();
		for item in data {
			result.push_str(item);
			result.push('\0');
		}

		result.into_bytes()
	}
}
