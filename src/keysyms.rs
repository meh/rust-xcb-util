use std::slice;

use xcb;
use ffi::keysyms::*;
use libc::{free, c_void};

pub struct KeyCode {
    ptr: *mut xcb::Keycode,
    index: isize,
}

impl Drop for KeyCode {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr as *mut c_void);
        }
    }
}

impl Iterator for KeyCode {
    type Item = xcb::Keycode;
    fn next(&mut self) -> Option<xcb::Keycode> {
        unsafe {
            match *self.ptr.offset(self.index) {
                0 => None, // pub const NO_SYMBOL: u32 = 0
                keycode => {
                    self.index += 1;
                    Some(keycode)
                }
            }
        }
    }
}

pub struct KeySymbols(*mut xcb_key_symbols_t);

impl KeySymbols {
    pub fn get_keysym(&self, keycode: xcb::Keycode, col: i32) -> xcb::Keysym {
        unsafe {
            xcb_key_symbols_get_keysym(self.0, keycode, col)
        }
    }

    pub fn get_keycode(&self, keysym: xcb::Keysym) -> KeyCode {
        unsafe {
            KeyCode{ptr: xcb_key_symbols_get_keycode(self.0, keysym), index: 0}
        }
    }

    pub fn press_lookup_keysym(&self, event: &xcb::KeyPressEvent, col: i32) -> xcb::Keysym {
        unsafe {
            xcb_key_press_lookup_keysym(self.0, event.ptr, col)
        }
    }

    pub fn release_lookup_keysym(&self, event: &xcb::KeyReleaseEvent, col: i32) -> xcb::Keysym {
        unsafe {
            xcb_key_release_lookup_keysym(self.0, event.ptr, col)
        }
    }

    pub fn refresh_keyboard_mapping(&self, event: &xcb::MappingNotifyEvent) -> i32 {
        unsafe {
            xcb_refresh_keyboard_mapping(self.0, event.ptr)
        }
    }
}

impl Drop for KeySymbols {
    fn drop(&mut self) {
        unsafe {
            xcb_key_symbols_free(self.0);
        }
    }
}

pub fn create(c: &xcb::Connection) -> KeySymbols {
    unsafe {
        KeySymbols(xcb_key_symbols_alloc(c.get_raw_conn()))
    }
}

pub fn is_keypad_key(keysym: xcb::Keysym) -> bool {
    unsafe {
        0 != xcb_is_keypad_key(keysym)
    }
}

pub fn is_private_keypad_key(keysym: xcb::Keysym) -> bool {
    unsafe {
        0 != xcb_is_private_keypad_key(keysym)
    }
}

pub fn is_cursor_key(keysym: xcb::Keysym) -> bool {
    unsafe {
        0 != xcb_is_cursor_key(keysym)
    }
}

pub fn is_pf_key(keysym: xcb::Keysym) -> bool {
    unsafe {
        0 != xcb_is_pf_key(keysym)
    }
}

pub fn is_function_key(keysym: xcb::Keysym) -> bool {
    unsafe {
        0 != xcb_is_function_key(keysym)
    }
}

pub fn is_misc_function_key(keysym: xcb::Keysym) -> bool {
    unsafe {
        0 != xcb_is_misc_function_key(keysym)
    }
}

pub fn is_modifier_key(keysym: xcb::Keysym) -> bool {
    unsafe {
        0 != xcb_is_modifier_key(keysym)
    }
}
