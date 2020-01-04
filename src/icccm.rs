use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::ffi::CStr;

use xcb;
use xcb::ffi::*;
use ffi::icccm::*;
use util::utf8;

macro_rules! property {
	(checked $name:ident with $func:ident -> $conn:expr, $cookie:expr) => (unsafe {
		$name(xcb::GetPropertyCookie {
			cookie:  $cookie,
			conn:    $conn,
			checked: true,
		}, $func)
	});

	(unchecked $name:ident with $func:ident -> $conn:expr, $cookie:expr) => (unsafe {
		$name(xcb::GetPropertyCookie {
			cookie:  $cookie,
			conn:    $conn,
			checked: false,
		}, $func)
	});

	(checked $name:ident -> $conn:expr, $cookie:expr) => (unsafe {
		$name(xcb::GetPropertyCookie {
			cookie:  $cookie,
			conn:    $conn,
			checked: true,
		})
	});

	(unchecked $name:ident -> $conn:expr, $cookie:expr) => (unsafe {
		$name(xcb::GetPropertyCookie {
			cookie:  $cookie,
			conn:    $conn,
			checked: false,
		})
	});
}

pub type WmState = xcb_icccm_wm_state_t;
pub const WM_STATE_WITHDRAWN: WmState = XCB_ICCCM_WM_STATE_WITHDRAWN;
pub const WM_STATE_NORMAL:    WmState = XCB_ICCCM_WM_STATE_NORMAL;
pub const WM_STATE_ICONIC:    WmState = XCB_ICCCM_WM_STATE_ICONIC;

define!(cookie GetTextPropertyCookie for xcb_icccm_get_text_property_reply_t => GetTextPropertyReply);
define!(reply GetTextPropertyReply for xcb_icccm_get_text_property_reply_t with xcb_icccm_get_text_property_reply_wipe);

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

pub fn get_text_property(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetTextPropertyCookie {
	property!(checked GetTextPropertyCookie with xcb_icccm_get_text_property_reply -> c,
		xcb_icccm_get_text_property(c.get_raw_conn(), window, property))
}

pub fn get_text_property_unchecked(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetTextPropertyCookie {
	property!(unchecked GetTextPropertyCookie with xcb_icccm_get_text_property_reply -> c,
		xcb_icccm_get_text_property_unchecked(c.get_raw_conn(), window, property))
}

pub fn set_wm_name<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let name = name.as_ref();

	void!(unchecked -> c,
		xcb_icccm_set_wm_name(c.get_raw_conn(), window, xcb::ATOM_STRING, 8,
			name.len() as u32, name.as_bytes().as_ptr() as *const _))
}

pub fn set_wm_name_checked<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let name = name.as_ref();

	void!(checked -> c,
		xcb_icccm_set_wm_name_checked(c.get_raw_conn(), window, xcb::ATOM_STRING, 8,
			name.len() as u32, name.as_bytes().as_ptr() as *const _))
}

pub fn get_wm_name(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	property!(checked GetTextPropertyCookie with xcb_icccm_get_wm_name_reply -> c,
		xcb_icccm_get_wm_name(c.get_raw_conn(), window))
}

pub fn set_wm_icon_name<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	let name = name.as_ref();

	void!(unchecked -> c,
		xcb_icccm_set_wm_icon_name(c.get_raw_conn(), window, encoding, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _))
}

pub fn set_wm_icon_name_checked<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	let name = name.as_ref();

	void!(checked -> c,
		xcb_icccm_set_wm_icon_name_checked(c.get_raw_conn(), window, encoding, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _))
}

pub fn get_wm_icon_name(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	property!(checked GetTextPropertyCookie with xcb_icccm_get_wm_icon_name_reply -> c,
		xcb_icccm_get_wm_icon_name(c.get_raw_conn(), window))
}

pub fn get_wm_icon_name_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	property!(unchecked GetTextPropertyCookie with xcb_icccm_get_wm_icon_name_reply -> c,
		xcb_icccm_get_wm_icon_name_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmColormapWindowsCookie with xcb_icccm_get_wm_colormap_windows_reply => GetWmColormapWindowsReply);
define!(reply GetWmColormapWindowsReply for xcb_icccm_get_wm_colormap_windows_reply_t with xcb_icccm_get_wm_colormap_windows_reply_wipe);

impl GetWmColormapWindowsReply {
	pub fn windows(&self) -> &[xcb::Window] {
		unsafe {
			slice::from_raw_parts(self.0.windows as *mut _, self.0.windows_len as usize)
		}
	}
}

pub fn set_wm_colormap_windows<'a>(c: &'a xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_icccm_set_wm_colormap_windows(c.get_raw_conn(), window, colormap_windows,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn set_wm_colormap_windows_checked<'a>(c: &'a xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_icccm_set_wm_colormap_windows_checked(c.get_raw_conn(), window, colormap_windows,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn get_wm_colormap_windows(c: &xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom) -> GetWmColormapWindowsCookie {
	property!(checked GetWmColormapWindowsCookie -> c,
		xcb_icccm_get_wm_colormap_windows(c.get_raw_conn(), window, colormap_windows))
}

pub fn get_wm_colormap_windows_unchecked(c: &xcb::Connection, window: xcb::Window, colormap_windows: xcb::Atom) -> GetWmColormapWindowsCookie {
	property!(unchecked GetWmColormapWindowsCookie -> c,
		xcb_icccm_get_wm_colormap_windows_unchecked(c.get_raw_conn(), window, colormap_windows))
}

pub fn set_wm_client_machine<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	let name = name.as_ref();

	void!(unchecked -> c,
		xcb_icccm_set_wm_client_machine(c.get_raw_conn(), window, encoding, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _))
}

pub fn set_wm_client_machine_checked<T: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, encoding: xcb::Atom, format: u8, name: T) -> xcb::VoidCookie {
	let name = name.as_ref();

	void!(checked -> c,
		xcb_icccm_set_wm_client_machine_checked(c.get_raw_conn(), window, encoding, format,
			name.len() as u32, name.as_bytes().as_ptr() as *const _))
}

pub fn get_wm_client_machine(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	property!(checked GetTextPropertyCookie with xcb_icccm_get_wm_client_machine_reply -> c,
		xcb_icccm_get_wm_client_machine(c.get_raw_conn(), window))
}

pub fn get_wm_client_machine_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetTextPropertyCookie {
	property!(unchecked GetTextPropertyCookie with xcb_icccm_get_wm_client_machine_reply -> c,
		xcb_icccm_get_wm_client_machine_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmClassCookie with xcb_icccm_get_wm_class_reply => GetWmClassReply);
define!(reply GetWmClassReply for xcb_icccm_get_wm_class_reply_t with xcb_icccm_get_wm_class_reply_wipe);

impl GetWmClassReply {
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

pub fn set_wm_class<A: AsRef<str>, B: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, class: A, instance: B) -> xcb::VoidCookie {
	let value = utf8::from(vec![class.as_ref(), instance.as_ref()]);

	void!(unchecked -> c,
		xcb_icccm_set_wm_class(c.get_raw_conn(), window,
			value.len() as u32, value.as_ptr() as *const _))
}

pub fn set_wm_class_checked<A: AsRef<str>, B: AsRef<str>>(c: &xcb::Connection, window: xcb::Window, class: A, instance: B) -> xcb::VoidCookie {
	let value = utf8::from(vec![class.as_ref(), instance.as_ref()]);

	void!(checked -> c,
		xcb_icccm_set_wm_class_checked(c.get_raw_conn(), window,
			value.len() as u32, value.as_ptr() as *const _))
}

pub fn get_wm_class(c: &xcb::Connection, window: xcb::Window) -> GetWmClassCookie {
	property!(checked GetWmClassCookie -> c,
		xcb_icccm_get_wm_class(c.get_raw_conn(), window))
}

pub fn get_wm_class_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetWmClassCookie {
	property!(unchecked GetWmClassCookie -> c,
		xcb_icccm_get_wm_class_unchecked(c.get_raw_conn(), window))
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

define!(cookie GetWmSizeHintsCookie with xcb_icccm_get_wm_size_hints_reply => SizeHints);

pub fn set_wm_size_hints<'a>(c: &'a xcb::Connection, window: xcb::Window, property: xcb::Atom, hints: &SizeHints) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_icccm_set_wm_size_hints(c.get_raw_conn(), window, property, &hints.0))
}

pub fn set_wm_size_hints_checked<'a>(c: &'a xcb::Connection, window: xcb::Window, property: xcb::Atom, hints: &SizeHints) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_icccm_set_wm_size_hints_checked(c.get_raw_conn(), window, property, &hints.0))
}

pub fn get_wm_size_hints(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetWmSizeHintsCookie {
	property!(checked GetWmSizeHintsCookie -> c,
		xcb_icccm_get_wm_size_hints(c.get_raw_conn(), window, property))
}

pub fn get_wm_size_hints_unchecked(c: &xcb::Connection, window: xcb::Window, property: xcb::Atom) -> GetWmSizeHintsCookie {
	property!(unchecked GetWmSizeHintsCookie -> c,
		xcb_icccm_get_wm_size_hints_unchecked(c.get_raw_conn(), window, property))
}

pub fn set_wm_normal_hints<'a>(c: &'a xcb::Connection, window: xcb::Window, hints: &SizeHints) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_icccm_set_wm_normal_hints(c.get_raw_conn(), window, &hints.0))
}

pub fn set_wm_normal_hints_checked<'a>(c: &'a xcb::Connection, window: xcb::Window, hints: &SizeHints) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_icccm_set_wm_normal_hints_checked(c.get_raw_conn(), window, &hints.0))
}

pub fn get_wm_normal_hints(c: &xcb::Connection, window: xcb::Window) -> GetWmSizeHintsCookie {
	property!(checked GetWmSizeHintsCookie -> c,
		xcb_icccm_get_wm_normal_hints(c.get_raw_conn(), window))
}

pub fn get_wm_normal_hints_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetWmSizeHintsCookie {
	property!(unchecked GetWmSizeHintsCookie -> c,
		xcb_icccm_get_wm_normal_hints_unchecked(c.get_raw_conn(), window))
}

pub struct WmHints(xcb_icccm_wm_hints_t);
pub struct WmHintsBuilder(xcb_icccm_wm_hints_t);

impl WmHints {
	pub fn empty() -> WmHintsBuilder {
		unsafe {
			WmHintsBuilder(mem::zeroed())
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

impl WmHintsBuilder {
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
			xcb_icccm_wm_hints_set_icon_pixmap(&mut self.0, icon);
		}

		self
	}

	pub fn icon_mask(mut self, icon: xcb::Pixmap) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_icon_mask(&mut self.0, icon);
		}

		self
	}

	pub fn icon_window(mut self, icon: xcb::Window) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_icon_window(&mut self.0, icon);
		}

		self
	}

	pub fn window_group(mut self, group: xcb::Window) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_window_group(&mut self.0, group);
		}

		self
	}

	pub fn is_urgent(mut self) -> Self {
		unsafe {
			xcb_icccm_wm_hints_set_urgency(&mut self.0);
		}

		self
	}

	pub fn build(self) -> WmHints {
		WmHints(self.0)
	}
}

define!(cookie GetWmHintsCookie with xcb_icccm_get_wm_hints_reply => WmHints);

pub fn set_wm_hints<'a>(c: &'a xcb::Connection, window: xcb::Window, hints: &WmHints) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_icccm_set_wm_hints(c.get_raw_conn(), window, &hints.0))
}

pub fn set_wm_hints_checked<'a>(c: &'a xcb::Connection, window: xcb::Window, hints: &WmHints) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_icccm_set_wm_hints_checked(c.get_raw_conn(), window, &hints.0))
}

pub fn get_wm_hints(c: &xcb::Connection, window: xcb::Window) -> GetWmHintsCookie {
	property!(checked GetWmHintsCookie -> c,
		xcb_icccm_get_wm_hints(c.get_raw_conn(), window))
}

pub fn get_wm_hints_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetWmHintsCookie {
	property!(unchecked GetWmHintsCookie -> c,
		xcb_icccm_get_wm_hints_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmProtocolsCookie with xcb_icccm_get_wm_protocols_reply => GetWmProtocolsReply);
define!(reply GetWmProtocolsReply for xcb_icccm_get_wm_protocols_reply_t with xcb_icccm_get_wm_protocols_reply_wipe);

impl GetWmProtocolsReply {
	pub fn atoms(&self) -> &[xcb::Atom] {
		unsafe {
			slice::from_raw_parts(self.0.atoms as *mut xcb::Atom, self.0.atoms_len as usize)
		}
	}
}

pub fn set_wm_protocols<'a>(c: &'a xcb::Connection, window: xcb::Window, protocols: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_icccm_set_wm_protocols(c.get_raw_conn(), window, protocols,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn set_wm_protocols_checked<'a>(c: &'a xcb::Connection, window: xcb::Window, protocols: xcb::Atom, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_icccm_set_wm_protocols_checked(c.get_raw_conn(), window, protocols,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn get_wm_protocols(c: &xcb::Connection, window: xcb::Window, protocols: xcb::Atom) -> GetWmProtocolsCookie {
	property!(checked GetWmProtocolsCookie -> c,
		xcb_icccm_get_wm_protocols(c.get_raw_conn(), window, protocols))
}

pub fn get_wm_protocols_unchecked(c: &xcb::Connection, window: xcb::Window, protocols: xcb::Atom) -> GetWmProtocolsCookie {
	property!(unchecked GetWmProtocolsCookie -> c,
		xcb_icccm_get_wm_protocols_unchecked(c.get_raw_conn(), window, protocols))
}

pub struct GetWmStateCookie<'a>(xcb::GetPropertyCookie<'a>);
pub struct GetWmStateReply(xcb::GetPropertyReply);

impl<'a> GetWmStateCookie<'a> {
	pub fn get_reply(self) -> Result<GetWmStateReply, xcb::GenericError> {
		let reply = self.0.get_reply()?;
		
		if reply.type_() == xcb::ATOM_NONE {
			Err(xcb::GenericError { ptr: ptr::null_mut() })
		}
		else {
			Ok(GetWmStateReply(reply))
		}
	}
}

impl GetWmStateReply {
	pub fn state(&self) -> WmState {
		self.0.value()[0]
	}

	pub fn icon(&self) -> xcb::Window {
		self.0.value()[1]
	}
}

pub fn set_wm_state(c: &xcb::Connection, window: xcb::Window, state: WmState, icon: xcb::Window) -> xcb::VoidCookie {
	let atom = xcb::intern_atom_unchecked(c, false, "WM_STATE").get_reply().unwrap().atom();
	xcb::change_property(c, xcb::CHANGE_PROPERTY as u8, window, atom, atom, 32, &[state as i32, icon as i32])
}

pub fn set_wm_state_checked(c: &xcb::Connection, window: xcb::Window, state: WmState, icon: xcb::Window) -> xcb::VoidCookie {
	let atom = xcb::intern_atom_unchecked(c, false, "WM_STATE").get_reply().unwrap().atom();
	xcb::change_property_checked(c, xcb::CHANGE_PROPERTY as u8, window, atom, atom, 32, &[state as i32, icon as i32])
}

pub fn get_wm_state(c: &xcb::Connection, window: xcb::Window) -> GetWmStateCookie {
	let atom = xcb::intern_atom_unchecked(c, false, "WM_STATE").get_reply().unwrap().atom();
	GetWmStateCookie(xcb::get_property(c, false, window, atom, atom, 0, 2))
}

pub fn get_wm_state_unchecked(c: &xcb::Connection, window: xcb::Window) -> GetWmStateCookie {
	let atom = xcb::intern_atom_unchecked(c, false, "WM_STATE").get_reply().unwrap().atom();
	GetWmStateCookie(xcb::get_property_unchecked(c, false, window, atom, atom, 0, 2))
}
