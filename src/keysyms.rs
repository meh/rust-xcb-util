use libc::c_int;

use xcb;
use ffi::keysyms::*;

pub struct KeySymbols<'a> {
	#[allow(dead_code)]
	// Keep a reference to the xcb::Connection to ensure we live long enough.
	conn: &'a xcb::Connection,
	keysyms: *mut xcb_key_symbols_t,
}

impl<'a> Drop for KeySymbols<'a> {
	fn drop(&mut self) {
		unsafe {
			xcb_key_symbols_free(self.keysyms);
		}
	}
}

impl<'a> KeySymbols<'a> {
	pub fn new(conn: &'a xcb::Connection) -> Result<KeySymbols<'a>, ()> {
		let keysyms = unsafe { xcb_key_symbols_alloc(conn.get_raw_conn()) };
		if keysyms.is_null() {
			// We don't have any details on the error, so don't provide any.
			return Err(());
		}
		Ok(KeySymbols { conn, keysyms })
	}

	pub fn get_keysym(&self, keycode: xcb::Keycode, col: i32) -> xcb::Keysym {
		unsafe {
			xcb_key_symbols_get_keysym(self.keysyms, keycode, col as c_int)
		}
	}

	pub fn get_keycode(&self, keysym: xcb::Keysym) -> xcb::Keycode {
		unsafe {
			*xcb_key_symbols_get_keycode(self.keysyms, keysym)
		}
	}

	pub fn key_press_lookup_keysym(&self, event: &xcb::KeyPressEvent, col: i32) -> xcb::Keysym {
		unsafe {
			xcb_key_press_lookup_keysym(self.keysyms, event.ptr, col as c_int)
		}
	}

	pub fn key_release_lookup_keysym(&self, event: &xcb::KeyReleaseEvent, col: i32) -> xcb::Keysym {
		unsafe {
			xcb_key_release_lookup_keysym(self.keysyms, event.ptr, col as c_int)
		}
	}

	pub fn refresh_keyboard_mapping(&self, event: &xcb::MappingNotifyEvent) {
		unsafe {
			xcb_refresh_keyboard_mapping(self.keysyms, event.ptr);
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
