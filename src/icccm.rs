use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::ffi::CStr;

use xcb;
use xcb::ffi::*;
use ffi::icccm::*;

pub struct GetTextPropertyCookie(xcb::GetPropertyCookie,
	unsafe extern "C" fn(*mut xcb_connection_t, xcb_get_property_cookie_t, *mut xcb_icccm_get_text_property_reply_t, *mut *mut xcb_generic_error_t) -> u8);

impl GetTextPropertyCookie {
	pub fn get_reply(&self) -> Result<GetTextPropertyReply, xcb::GenericError> {
		unsafe {
			if self.0.checked {
				let mut err: *mut xcb_generic_error_t = ptr::null_mut();
				let mut reply = mem::uninitialized();
				self.1(self.0.conn, self.0.cookie, &mut reply, &mut err);

				if err.is_null() {
					Ok(GetTextPropertyReply(reply))
				}
				else {
					Err(xcb::GenericError { ptr: err })
				}
			}
			else {
				let mut reply = mem::uninitialized();
				self.1(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

				Ok(GetTextPropertyReply(reply))
			}
		}
	}
}

pub struct GetTextPropertyReply(xcb_icccm_get_text_property_reply_t);

impl GetTextPropertyReply {
	pub fn encoding(&self) -> xcb::Atom {
		self.0.encoding as xcb::Atom
	}

	pub fn name(&self) -> &str {
		unsafe {
			str::from_utf8_unchecked(slice::from_raw_parts(
				self.0.name as *mut u8, self.0.name_len as usize))
		}
	}

	pub fn format(&self) -> u8 {
		self.0.format
	}
}

impl Drop for GetTextPropertyReply {
	fn drop(&mut self) {
		unsafe {
			xcb_icccm_get_text_property_reply_wipe(&mut self.0);
		}
	}
}

pub fn get_text_property(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetTextPropertyCookie {
	unsafe {
		let cookie = xcb_icccm_get_text_property(
			c.get_raw_conn(), window as xcb_window_t, property as xcb_atom_t);

		GetTextPropertyCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		}, xcb_icccm_get_text_property_reply)
	}
}

pub fn get_text_property_unchecked(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetTextPropertyCookie {
	unsafe {
		let cookie = xcb_icccm_get_text_property_unchecked(
			c.get_raw_conn(), window as xcb_window_t, property as xcb_atom_t);

		GetTextPropertyCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		}, xcb_icccm_get_text_property_reply)
	}
}

pub fn set_wm_name<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_name(
			c.get_raw_conn(), window as xcb_window_t, encoding as xcb_atom_t, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		}
	}
}

pub fn set_wm_name_checked<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_name_checked(
			c.get_raw_conn(), window as xcb_window_t, encoding as xcb_atom_t, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		}
	}
}

pub fn get_wm_name(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_name(
			c.get_raw_conn(), window as xcb_window_t);

		GetTextPropertyCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		}, xcb_icccm_get_wm_name_reply)
	}
}

pub fn set_wm_icon_name<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_icon_name(
			c.get_raw_conn(), window as xcb_window_t, encoding as xcb_atom_t, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		}
	}
}

pub fn set_wm_icon_name_checked<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_icon_name_checked(
			c.get_raw_conn(), window as xcb_window_t, encoding as xcb_atom_t, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		}
	}
}

pub fn get_wm_icon_name(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_icon_name(
			c.get_raw_conn(), window as xcb_window_t);

		GetTextPropertyCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		}, xcb_icccm_get_wm_icon_name_reply)
	}
}

pub fn get_wm_icon_name_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_icon_name_unchecked(
			c.get_raw_conn(), window as xcb_window_t);

		GetTextPropertyCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		}, xcb_icccm_get_wm_icon_name_reply)
	}
}

pub struct GetWMColormapWindowsCookie(xcb::GetPropertyCookie);

impl GetWMColormapWindowsCookie {
	pub fn get_reply(&self) -> Result<GetWMColormapWindowsReply, xcb::GenericError> {
		unsafe {
			if self.0.checked {
				let mut err: *mut xcb_generic_error_t = ptr::null_mut();
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_colormap_windows_reply(self.0.conn, self.0.cookie, &mut reply, &mut err);

				if err.is_null() {
					Ok(GetWMColormapWindowsReply(reply))
				}
				else {
					Err(xcb::GenericError { ptr: err })
				}
			}
			else {
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_colormap_windows_reply(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

				Ok(GetWMColormapWindowsReply(reply))
			}
		}
	}
}

pub struct GetWMColormapWindowsReply(xcb_icccm_get_wm_colormap_windows_reply_t);

impl GetWMColormapWindowsReply {
	pub fn windows(&self) -> &[xcb::Window] {
		unsafe {
			slice::from_raw_parts(self.0.windows as *mut xcb::Window, self.0.windows_len as usize)
		}
	}
}

impl Drop for GetWMColormapWindowsReply {
	fn drop(&mut self) {
		unsafe {
			xcb_icccm_get_wm_colormap_windows_reply_wipe(&mut self.0);
		}
	}
}

pub fn set_wm_colormap_windows(c: &xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_colormap_windows(
			c.get_raw_conn(), window as xcb_window_t, colormap_windows as xcb_atom_t,
			list.len() as u32, list.as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		}
	}
}

pub fn set_wm_colormap_windows_checked(c: &xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_colormap_windows_checked(
			c.get_raw_conn(), window as xcb_window_t, colormap_windows as xcb_atom_t,
			list.len() as u32, list.as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		}
	}
}

pub fn get_wm_colormap_windows(c: &xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom) -> GetWMColormapWindowsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_colormap_windows(
			c.get_raw_conn(), window as xcb_window_t, colormap_windows as xcb_atom_t);

		GetWMColormapWindowsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		})
	}
}

pub fn get_wm_colormap_windows_unchecked(c: &xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom) -> GetWMColormapWindowsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_colormap_windows_unchecked(
			c.get_raw_conn(), window as xcb_window_t, colormap_windows as xcb_atom_t);

		GetWMColormapWindowsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		})
	}
}

pub fn set_wm_client_machine<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_client_machine(
			c.get_raw_conn(), window as xcb_window_t, encoding as xcb_atom_t, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		}
	}
}

pub fn set_wm_client_machine_checked<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_client_machine_checked(
			c.get_raw_conn(), window as xcb_window_t, encoding as xcb_atom_t, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		}
	}
}

pub fn get_wm_client_machine(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_client_machine(
			c.get_raw_conn(), window as xcb_window_t);

		GetTextPropertyCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		}, xcb_icccm_get_wm_client_machine_reply)
	}
}

pub fn get_wm_client_machine_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_client_machine_unchecked(
			c.get_raw_conn(), window as xcb_window_t);

		GetTextPropertyCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		}, xcb_icccm_get_wm_client_machine_reply)
	}
}

pub struct GetWMClassCookie(xcb::GetPropertyCookie);

impl GetWMClassCookie {
	pub fn get_reply(&self) -> Result<GetWMClassReply, xcb::GenericError> {
		unsafe {
			if self.0.checked {
				let mut err: *mut xcb_generic_error_t = ptr::null_mut();
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_class_reply(self.0.conn, self.0.cookie, &mut reply, &mut err);

				if err.is_null() {
					Ok(GetWMClassReply(reply))
				}
				else {
					Err(xcb::GenericError { ptr: err })
				}
			}
			else {
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_class_reply(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

				Ok(GetWMClassReply(reply))
			}
		}
	}
}

pub struct GetWMClassReply(xcb_icccm_get_wm_class_reply_t);

impl GetWMClassReply {
	pub fn instance(&self) -> &str {
		unsafe {
			CStr::from_ptr(self.0.instance_name).to_str().unwrap()
		}
	}

	pub fn class(&self) -> &str {
		unsafe {
			CStr::from_ptr(self.0.class_name).to_str().unwrap()
		}
	}
}

impl Drop for GetWMClassReply {
	fn drop(&mut self) {
		unsafe {
			xcb_icccm_get_wm_class_reply_wipe(&mut self.0);
		}
	}
}

pub fn set_wm_class<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_class(
			c.get_raw_conn(), window as xcb_window_t,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		}
	}
}

pub fn set_wm_class_checked<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	unsafe {
		let name   = name.as_ref();
		let cookie = xcb_icccm_set_wm_class_checked(
			c.get_raw_conn(), window as xcb_window_t,
			name.len() as u32, name.as_bytes().as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		}
	}
}

pub fn get_wm_class(c: &xcb::Connection, window: xcb::Window) -> GetWMClassCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_class(
			c.get_raw_conn(), window as xcb_window_t);

		GetWMClassCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		})
	}
}

pub fn get_wm_class_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetWMClassCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_class_unchecked(
			c.get_raw_conn(), window as xcb_window_t);

		GetWMClassCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		})
	}
}

pub struct SizeHints(xcb_size_hints_t);
pub struct SizeHintsBuilder(xcb_size_hints_t);

impl SizeHints {
	pub fn empty() -> SizeHintsBuilder {
		unsafe {
			SizeHintsBuilder(mem::zeroed())
		}
	}

	pub fn position(&self) -> Option<(i32, i32)> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_P_POSITION == 1 {
			Some((self.0.x, self.0.y))
		}
		else {
			None
		}
	}

	pub fn size(&self) -> Option<(i32, i32)> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_P_SIZE == 1 {
			Some((self.0.width, self.0.height))
		}
		else {
			None
		}
	}

	pub fn min_size(&self) -> Option<(i32, i32)> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_P_MIN_SIZE == 1 {
			Some((self.0.min_width, self.0.min_height))
		}
		else {
			None
		}
	}

	pub fn max_size(&self) -> Option<(i32, i32)> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_P_MAX_SIZE == 1 {
			Some((self.0.max_width, self.0.max_height))
		}
		else {
			None
		}
	}


	pub fn resize(&self) -> Option<(i32, i32)> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_P_RESIZE_INC == 1 {
			Some((self.0.width_inc, self.0.height_inc))
		}
		else {
			None
		}
	}

	pub fn aspect(&self) -> Option<((i32, i32), (i32, i32))> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_P_ASPECT == 1 {
			Some(((self.0.min_aspect_num, self.0.min_aspect_den), (self.0.max_aspect_num, self.0.max_aspect_den)))
		}
		else {
			None
		}
	}

	pub fn base(&self) -> Option<(i32, i32)> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_BASE_SIZE == 1 {
			Some((self.0.base_width, self.0.base_height))
		}
		else {
			None
		}
	}

	pub fn gravity(&self) -> Option<xcb::Gravity> {
		if self.0.flags & XCB_ICCCM_SIZE_HINT_P_WIN_GRAVITY == 1 {
			Some(self.0.win_gravity as xcb::Gravity)
		}
		else {
			None
		}
	}
}

impl SizeHintsBuilder {
	pub fn position(mut self, x: i32, y: i32) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_position(&mut self.0, 0, x, y);
		}

		self
	}

	pub fn size(mut self, width: i32, height: i32) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_size(&mut self.0, 0, width, height)
		}

		self
	}

	pub fn min_size(mut self, width: i32, height: i32) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_min_size(&mut self.0, width, height)
		}

		self
	}

	pub fn max_size(mut self, width: i32, height: i32) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_max_size(&mut self.0, width, height)
		}

		self
	}

	pub fn resize(mut self, width: i32, height: i32) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_resize_inc(&mut self.0, width, height)
		}

		self
	}

	pub fn aspect(mut self, min: (i32, i32), max: (i32, i32)) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_aspect(&mut self.0, min.0, min.1, max.0, max.1)
		}

		self
	}

	pub fn base(mut self, width: i32, height: i32) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_base_size(&mut self.0, width, height)
		}

		self
	}

	pub fn gravity(mut self, gravity: xcb::Gravity) -> Self {
		unsafe {
			xcb_icccm_size_hints_set_win_gravity(&mut self.0, gravity)
		}

		self
	}

	pub fn build(self) -> SizeHints {
		SizeHints(self.0)
	}
}

pub struct GetWMSizeHintsCookie(xcb::GetPropertyCookie);

impl GetWMSizeHintsCookie {
	pub fn get_reply(&self) -> Result<SizeHints, xcb::GenericError> {
		unsafe {
			if self.0.checked {
				let mut err: *mut xcb_generic_error_t = ptr::null_mut();
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_size_hints_reply(self.0.conn, self.0.cookie, &mut reply, &mut err);

				if err.is_null() {
					Ok(SizeHints(reply))
				}
				else {
					Err(xcb::GenericError { ptr: err })
				}
			}
			else {
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_size_hints_reply(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

				Ok(SizeHints(reply))
			}
		}
	}
}

pub fn set_wm_size_hints(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom, hints: &SizeHints) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_size_hints(
			c.get_raw_conn(), window as xcb_window_t, property as xcb_atom_t, &hints.0);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		}
	}
}

pub fn set_wm_size_hints_checked(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom, hints: &SizeHints) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_size_hints_checked(
			c.get_raw_conn(), window as xcb_window_t, property as xcb_atom_t, &hints.0);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		}
	}
}

pub fn get_wm_size_hints(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetWMSizeHintsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_size_hints(
			c.get_raw_conn(), window as xcb_window_t, property as xcb_atom_t);

		GetWMSizeHintsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		})
	}
}

pub fn get_wm_size_hints_unchecked(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetWMSizeHintsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_size_hints_unchecked(
			c.get_raw_conn(), window as xcb_window_t, property as xcb_atom_t);

		GetWMSizeHintsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		})
	}
}

pub fn set_wm_normal_hints(c: &xcb::Connection, window: xcb::Window, hints: &SizeHints) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_normal_hints(
			c.get_raw_conn(), window as xcb_window_t, &hints.0);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		}
	}
}

pub fn set_wm_normal_hints_checked(c: &xcb::Connection, window: xcb::Window, hints: &SizeHints) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_normal_hints_checked(
			c.get_raw_conn(), window as xcb_window_t, &hints.0);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		}
	}
}

pub fn get_wm_normal_hints(c: &xcb::Connection, window: xcb::Window) -> GetWMSizeHintsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_normal_hints(
			c.get_raw_conn(), window as xcb_window_t);

		GetWMSizeHintsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		})
	}
}

pub fn get_wm_normal_hints_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetWMSizeHintsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_normal_hints_unchecked(
			c.get_raw_conn(), window as xcb_window_t);

		GetWMSizeHintsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		})
	}
}

pub struct WMHints(xcb_icccm_wm_hints_t);
pub struct WMHintsBuilder(xcb_icccm_wm_hints_t);

impl WMHints {
	pub fn empty() -> WMHintsBuilder {
		unsafe {
			WMHintsBuilder(mem::zeroed())
		}
	}

	pub fn input(&self) -> Option<bool> {
		if self.0.flags & XCB_ICCCM_WM_HINT_INPUT == 1 {
			Some(self.0.input != 0)
		}
		else {
			None
		}
	}

	pub fn is_iconic(&self) -> bool {
		self.0.flags & XCB_ICCCM_WM_HINT_INPUT == 1
			&& self.0.initial_state == XCB_ICCCM_WM_STATE_ICONIC
	}

	pub fn is_normal(&self) -> bool {
		self.0.flags & XCB_ICCCM_WM_HINT_INPUT == 1
			&& self.0.initial_state == XCB_ICCCM_WM_STATE_NORMAL
	}

	pub fn is_withdrawn(&self) -> bool {
		self.0.flags & XCB_ICCCM_WM_HINT_INPUT == 1
			&& self.0.initial_state == XCB_ICCCM_WM_STATE_WITHDRAWN
	}

	pub fn is_none(&self) -> bool {
		self.0.flags & XCB_ICCCM_WM_HINT_INPUT != 1 || (
			self.0.initial_state != XCB_ICCCM_WM_STATE_ICONIC &&
			self.0.initial_state != XCB_ICCCM_WM_STATE_NORMAL &&
			self.0.initial_state != XCB_ICCCM_WM_STATE_WITHDRAWN)
	}

	pub fn icon_pixmap(&self) -> Option<xcb::Pixmap> {
		if self.0.flags & XCB_ICCCM_WM_HINT_ICON_PIXMAP == 1 {
			Some(self.0.icon_pixmap as xcb::Pixmap)
		}
		else {
			None
		}
	}

	pub fn icon_mask(&self) -> Option<xcb::Pixmap> {
		if self.0.flags & XCB_ICCCM_WM_HINT_ICON_MASK == 1 {
			Some(self.0.icon_mask as xcb::Pixmap)
		}
		else {
			None
		}
	}

	pub fn icon_window(&self) -> Option<xcb::Window> {
		if self.0.flags & XCB_ICCCM_WM_HINT_ICON_WINDOW == 1 {
			Some(self.0.icon_window as xcb::Window)
		}
		else {
			None
		}
	}

	pub fn window_group(&self) -> Option<xcb::Window> {
		if self.0.flags & XCB_ICCCM_WM_HINT_WINDOW_GROUP == 1 {
			Some(self.0.window_group as xcb::Window)
		}
		else {
			None
		}
	}

	pub fn is_urgent(&self) -> Option<bool> {
		if self.0.flags & XCB_ICCCM_WM_HINT_X_URGENCY == 1 {
			Some(unsafe {
				xcb_icccm_wm_hints_get_urgency(&self.0) != 0
			})
		}
		else {
			None
		}
	}
}

impl WMHintsBuilder {
	pub fn input(mut self, value: bool) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_input(&mut self.0, value as u8);
		}

		self
	}

	pub fn is_iconic(mut self) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_iconic(&mut self.0);
		}

		self
	}

	pub fn is_normal(mut self) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_normal(&mut self.0);
		}

		self
	}

	pub fn is_withdrawn(mut self) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_withdrawn(&mut self.0);
		}

		self
	}

	pub fn is_none(mut self) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_none(&mut self.0);
		}

		self
	}

	pub fn icon_pixmap(mut self, icon: xcb::Pixmap) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_icon_pixmap(&mut self.0, icon as xcb_pixmap_t);
		}

		self
	}

	pub fn icon_mask(mut self, icon: xcb::Pixmap) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_icon_mask(&mut self.0, icon as xcb_pixmap_t);
		}

		self
	}

	pub fn icon_window(mut self, icon: xcb::Window) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_icon_window(&mut self.0, icon as xcb_window_t);
		}

		self
	}

	pub fn window_group(mut self, group: xcb::Window) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_window_group(&mut self.0, group as xcb_window_t);
		}

		self
	}

	pub fn is_urgent(mut self) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_urgency(&mut self.0);
		}

		self
	}

	pub fn build(self) -> WMHints {
		WMHints(self.0)
	}
}

pub struct GetWMHintsCookie(xcb::GetPropertyCookie);

impl GetWMHintsCookie {
	pub fn get_reply(&self) -> Result<WMHints, xcb::GenericError> {
		unsafe {
			if self.0.checked {
				let mut err: *mut xcb_generic_error_t = ptr::null_mut();
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_hints_reply(self.0.conn, self.0.cookie, &mut reply, &mut err);

				if err.is_null() {
					Ok(WMHints(reply))
				}
				else {
					Err(xcb::GenericError { ptr: err })
				}
			}
			else {
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_hints_reply(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

				Ok(WMHints(reply))
			}
		}
	}
}

pub fn set_wm_hints(c: &xcb::Connection, window: xcb::Window, hints: &WMHints) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_hints(
			c.get_raw_conn(), window as xcb_window_t, &hints.0);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		}
	}
}

pub fn set_wm_hints_checked(c: &xcb::Connection, window: xcb::Window, hints: &WMHints) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_hints_checked(
			c.get_raw_conn(), window as xcb_window_t, &hints.0);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		}
	}
}

pub fn get_wm_hints(c: &xcb::Connection, window: xcb::Window) -> GetWMHintsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_hints(
			c.get_raw_conn(), window as xcb_window_t);

		GetWMHintsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true,
		})
	}
}

pub fn get_wm_hints_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetWMHintsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_hints_unchecked(
			c.get_raw_conn(), window as xcb_window_t);

		GetWMHintsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false,
		})
	}
}

pub struct GetWMProtocolsCookie(xcb::GetPropertyCookie);

impl GetWMProtocolsCookie {
	pub fn get_reply(&self) -> Result<GetWMProtocolsReply, xcb::GenericError> {
		unsafe {
			if self.0.checked {
				let mut err: *mut xcb_generic_error_t = ptr::null_mut();
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_protocols_reply(self.0.conn, self.0.cookie, &mut reply, &mut err);

				if err.is_null() {
					Ok(GetWMProtocolsReply(reply))
				}
				else {
					Err(xcb::GenericError { ptr: err })
				}
			}
			else {
				let mut reply = mem::uninitialized();
				xcb_icccm_get_wm_protocols_reply(self.0.conn, self.0.cookie, &mut reply, ptr::null_mut());

				Ok(GetWMProtocolsReply(reply))
			}
		}
	}
}

pub struct GetWMProtocolsReply(xcb_icccm_get_wm_protocols_reply_t);

impl GetWMProtocolsReply {
	pub fn atoms(&self) -> &[xcb::Atom] {
		unsafe {
			slice::from_raw_parts(self.0.atoms as *mut xcb::Atom, self.0.atoms_len as usize)
		}
	}
}

impl Drop for GetWMProtocolsReply {
	fn drop(&mut self) {
		unsafe {
			xcb_icccm_get_wm_protocols_reply_wipe(&mut self.0);
		}
	}
}

pub fn set_wm_protocols(c: &xcb::Connection, window: xcb::Window, protocols: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_protocols(
			c.get_raw_conn(), window as xcb_window_t, protocols as xcb_atom_t,
			list.len() as u32, list.as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		}
	}
}

pub fn set_wm_protocols_checked(c: &xcb::Connection, window: xcb::Window, protocols: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie {
	unsafe {
		let cookie = xcb_icccm_set_wm_protocols_checked(
			c.get_raw_conn(), window as xcb_window_t, protocols as xcb_atom_t,
			list.len() as u32, list.as_ptr() as *const _);

		xcb::VoidCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		}
	}
}

pub fn get_wm_protocols(c: &xcb::Connection, window: xcb::Window, protocols: xcb::Atom) -> GetWMColormapWindowsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_protocols(
			c.get_raw_conn(), window as xcb_window_t, protocols as xcb_atom_t);

		GetWMColormapWindowsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: true
		})
	}
}

pub fn get_wm_protocols_unchecked(c: &xcb::Connection, window: xcb::Window, protocols: xcb::Atom) -> GetWMColormapWindowsCookie {
	unsafe {
		let cookie = xcb_icccm_get_wm_protocols_unchecked(
			c.get_raw_conn(), window as xcb_window_t, protocols as xcb_atom_t);

		GetWMColormapWindowsCookie(xcb::GetPropertyCookie {
			cookie:  cookie,
			conn:    c.get_raw_conn(),
			checked: false
		})
	}
}
