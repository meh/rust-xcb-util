use xcb;
use ffi::keysyms::*;
use libc::{free, c_void};

pub struct KeySymbols<'a> {
	ptr:  *mut xcb_key_symbols_t,
	conn: &'a xcb::Connection,
}

impl<'a> KeySymbols<'a> {
	pub fn new(c: &xcb::Connection) -> KeySymbols {
		unsafe {
			KeySymbols {
				ptr:  xcb_key_symbols_alloc(c.get_raw_conn()),
				conn: c,
			}
		}
	}

	pub fn get_keysym(&self, keycode: xcb::Keycode, col: i32) -> xcb::Keysym {
		unsafe {
			xcb_key_symbols_get_keysym(self.ptr, keycode, col)
		}
	}

	pub fn get_keycode(&self, keysym: xcb::Keysym) -> KeycodeIter {
		unsafe {
			KeycodeIter {
				ptr:   xcb_key_symbols_get_keycode(self.ptr, keysym),
				index: 0,
			}
		}
	}

	pub fn press_lookup_keysym(&self, event: &xcb::KeyPressEvent, col: i32) -> xcb::Keysym {
		unsafe {
			xcb_key_press_lookup_keysym(self.ptr, event.ptr, col)
		}
	}

	pub fn release_lookup_keysym(&self, event: &xcb::KeyReleaseEvent, col: i32) -> xcb::Keysym {
		unsafe {
			xcb_key_release_lookup_keysym(self.ptr, event.ptr, col)
		}
	}

	pub fn refresh_keyboard_mapping(&self, event: &xcb::MappingNotifyEvent) -> i32 {
		unsafe {
			xcb_refresh_keyboard_mapping(self.ptr, event.ptr)
		}
	}
}

impl<'a> Drop for KeySymbols<'a> {
	fn drop(&mut self) {
		unsafe {
			xcb_key_symbols_free(self.ptr);
		}
	}
}

pub struct KeycodeIter {
	ptr: *mut xcb::Keycode,
	index: isize,
}

impl Drop for KeycodeIter {
	fn drop(&mut self) {
		unsafe {
			free(self.ptr as *mut c_void);
		}
	}
}

impl Iterator for KeycodeIter {
	type Item = xcb::Keycode;

	fn next(&mut self) -> Option<xcb::Keycode> {
		unsafe {
			if self.ptr.is_null() {
				return None;
			}

			match *self.ptr.offset(self.index) {
				0 =>
					None,

				keycode => {
					self.index += 1;
					Some(keycode)
				}
			}
		}
	}
}

pub fn is_keypad_key(keysym: xcb::Keysym) -> bool {
	unsafe {
		xcb_is_keypad_key(keysym) != 0
	}
}

pub fn is_private_keypad_key(keysym: xcb::Keysym) -> bool {
	unsafe {
		xcb_is_private_keypad_key(keysym) != 0
	}
}

pub fn is_cursor_key(keysym: xcb::Keysym) -> bool {
	unsafe {
		xcb_is_cursor_key(keysym) != 0
	}
}

pub fn is_pf_key(keysym: xcb::Keysym) -> bool {
	unsafe {
		xcb_is_pf_key(keysym) != 0
	}
}

pub fn is_function_key(keysym: xcb::Keysym) -> bool {
	unsafe {
		xcb_is_function_key(keysym) != 0
	}
}

pub fn is_misc_function_key(keysym: xcb::Keysym) -> bool {
	unsafe {
		xcb_is_misc_function_key(keysym) != 0
	}
}

pub fn is_modifier_key(keysym: xcb::Keysym) -> bool {
	unsafe {
		xcb_is_modifier_key(keysym) != 0
	}
}
