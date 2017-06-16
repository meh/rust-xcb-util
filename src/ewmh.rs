use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::ops::{Deref, DerefMut};

use xcb;
use xcb::ffi::*;
use ffi::ewmh::*;
use libc::c_int;
use util::utf8;

pub type Coordinates = xcb_ewmh_coordinates_t;

impl Coordinates {
	pub fn x(&self) -> u32 {
		self.x
	}

	pub fn y(&self) -> u32 {
		self.y
	}
}

pub type Geometry = xcb_ewmh_geometry_t;

impl Geometry {
	pub fn x(&self) -> u32 {
		self.x
	}

	pub fn y(&self) -> u32 {
		self.y
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}
}

pub type StrutPartial = xcb_ewmh_wm_strut_partial_t;

impl StrutPartial {
	pub fn left(&self) -> u32 {
		self.left
	}

	pub fn right(&self) -> u32 {
		self.right
	}

	pub fn top(&self) -> u32 {
		self.top
	}

	pub fn bottom(&self) -> u32 {
		self.bottom
	}

	pub fn left_start_y(&self) -> u32 {
		self.left_start_y
	}

	pub fn left_end_y(&self) -> u32 {
		self.left_end_y
	}

	pub fn right_start_y(&self) -> u32 {
		self.right_start_y
	}

	pub fn right_end_y(&self) -> u32 {
		self.right_end_y
	}

	pub fn top_start_x(&self) -> u32 {
		self.top_start_x
	}

	pub fn top_end_x(&self) -> u32 {
		self.top_end_x
	}

	pub fn bottom_start_x(&self) -> u32 {
		self.bottom_start_x
	}

	pub fn bottom_end_x(&self) -> u32 {
		self.bottom_end_x
	}
}

pub type Extents = xcb_ewmh_get_extents_reply_t;

impl Extents {
	pub fn top(&self) -> u32 {
		self.top
	}

	pub fn bottom(&self) -> u32 {
		self.bottom
	}

	pub fn left(&self) -> u32 {
		self.left
	}

	pub fn right(&self) -> u32 {
		self.right
	}
}

pub struct WmIcon {
	width:  u32,
	height: u32,
	id:     u32,
}

impl WmIcon {
	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn id(&self) -> u32 {
		self.id
	}
}


pub type WmIconIterator = xcb_ewmh_wm_icon_iterator_t;

impl Iterator for WmIconIterator {
	type Item = WmIcon;

	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			if self.rem == 0 {
				None
			}
			else {
				let width  = self.width;
				let height = self.height;
				let id     = *self.data;

				xcb_ewmh_get_wm_icon_next(self);

				Some(WmIcon {
					width:  width,
					height: height,
					id:     id
				})
			}
		}
	}
}

pub type WmFullScreenMonitors = xcb_ewmh_get_wm_fullscreen_monitors_reply_t;

impl WmFullScreenMonitors {
	pub fn top(&self) -> u32 {
		self.top
	}

	pub fn bottom(&self) -> u32 {
		self.bottom
	}

	pub fn left(&self) -> u32 {
		self.left
	}

	pub fn right(&self) -> u32 {
		self.right
	}
}

pub type ClientSourceType = xcb_ewmh_client_source_type_t;
pub const CLIENT_SOURCE_TYPE_NONE:   ClientSourceType = 0;
pub const CLIENT_SOURCE_TYPE_NORMAL: ClientSourceType = 1;
pub const CLIENT_SOURCE_TYPE_OTHER:  ClientSourceType = 2;

pub type DesktopLayoutOrientation = xcb_ewmh_desktop_layout_orientation_t;
pub const ORIENTATION_HORZ: DesktopLayoutOrientation = 0;
pub const ORIENTATION_VERT: DesktopLayoutOrientation = 1;

pub type DesktopLayoutStartingCorner = xcb_ewmh_desktop_layout_starting_corner_t;
pub const TOP_LEFT:     DesktopLayoutStartingCorner = 0;
pub const TOP_RIGHT:    DesktopLayoutStartingCorner = 1;
pub const BOTTOM_RIGHT: DesktopLayoutStartingCorner = 2;
pub const BOTTOM_LEFT:  DesktopLayoutStartingCorner = 3;

pub type MoveResizeWindowFlags = xcb_ewmh_moveresize_window_opt_flags_t;
pub const MOVE_RESIZE_WINDOW_X:      MoveResizeWindowFlags = 1 << 8;
pub const MOVE_RESIZE_WINDOW_Y:      MoveResizeWindowFlags = 1 << 9;
pub const MOVE_RESIZE_WINDOW_WIDTH:  MoveResizeWindowFlags = 1 << 10;
pub const MOVE_RESIZE_WINDOW_HEIGHT: MoveResizeWindowFlags = 1 << 11;

pub type MoveResizeDirection = xcb_ewmh_moveresize_direction_t;
pub const MOVE_RESIZE_SIZE_TOPLEFT:     MoveResizeDirection = 0;
pub const MOVE_RESIZE_SIZE_TOP:         MoveResizeDirection = 1;
pub const MOVE_RESIZE_SIZE_TOPRIGHT:    MoveResizeDirection = 2;
pub const MOVE_RESIZE_SIZE_RIGHT:       MoveResizeDirection = 3;
pub const MOVE_RESIZE_SIZE_BOTTOMRIGHT: MoveResizeDirection = 4;
pub const MOVE_RESIZE_SIZE_BOTTOM:      MoveResizeDirection = 5;
pub const MOVE_RESIZE_SIZE_BOTTOMLEFT:  MoveResizeDirection = 6;
pub const MOVE_RESIZE_SIZE_LEFT:        MoveResizeDirection = 7;
pub const MOVE_RESIZE_MOVE:             MoveResizeDirection = 8;
pub const MOVE_RESIZE_SIZE_KEYBOARD:    MoveResizeDirection = 9;
pub const MOVE_RESIZE_MOVE_KEYBOARD:    MoveResizeDirection = 10;
pub const MOVE_RESIZE_CANCEL:           MoveResizeDirection = 11;

pub type StateAction = xcb_ewmh_wm_state_action_t;
pub const STATE_REMOVE: StateAction = 0;
pub const STATE_ADD:    StateAction = 1;
pub const STATE_TOGGLE: StateAction = 2;

pub struct Connection {
	xcb:  xcb::Connection,
	ewmh: xcb_ewmh_connection_t,
}

#[cfg(feature = "thread")]
unsafe impl<'a> Send for Connection { }
#[cfg(feature = "thread")]
unsafe impl<'a> Sync for Connection { }

impl Connection {
	pub fn connect(xcb: xcb::Connection) -> Result<Connection, (xcb::GenericError, xcb::Connection)> {
		unsafe {
			let mut ewmh = mem::uninitialized();
			let mut err: *mut xcb_generic_error_t = ptr::null_mut();

			let cookie = xcb_ewmh_init_atoms(xcb.get_raw_conn(), &mut ewmh);
			xcb_ewmh_init_atoms_replies(&mut ewmh, cookie, &mut err);

			if err.is_null() {
				Ok(Connection {
					xcb:  xcb,
					ewmh: ewmh,
				})
			}
			else {
				Err((xcb::GenericError { ptr: err }, xcb))
			}
		}
	}

	#[inline(always)]
	pub fn get_raw_conn(&self) -> *mut xcb_ewmh_connection_t {
		&self.ewmh as *const _ as *mut _
	}

	#[inline(always)]
	pub fn WM_CM(&self) -> &[xcb::Atom] {
		unsafe {
			slice::from_raw_parts(self.ewmh._NET_WM_CM_Sn, self.ewmh.nb_screens as usize)
		}
	}

	#[inline(always)]
	pub fn SUPPORTED(&self) -> xcb::Atom {
		self.ewmh._NET_SUPPORTED
	}

	#[inline(always)]
	pub fn CLIENT_LIST(&self) -> xcb::Atom {
		self.ewmh._NET_CLIENT_LIST
	}

	#[inline(always)]
	pub fn CLIENT_LIST_STACKING(&self) -> xcb::Atom {
		self.ewmh._NET_CLIENT_LIST_STACKING
	}

	#[inline(always)]
	pub fn NUMBER_OF_DESKTOPS(&self) -> xcb::Atom {
		self.ewmh._NET_NUMBER_OF_DESKTOPS
	}

	#[inline(always)]
	pub fn DESKTOP_GEOMETRY(&self) -> xcb::Atom {
		self.ewmh._NET_DESKTOP_GEOMETRY
	}

	#[inline(always)]
	pub fn DESKTOP_VIEWPORT(&self) -> xcb::Atom {
		self.ewmh._NET_DESKTOP_VIEWPORT
	}

	#[inline(always)]
	pub fn CURRENT_DESKTOP(&self) -> xcb::Atom {
		self.ewmh._NET_CURRENT_DESKTOP
	}

	#[inline(always)]
	pub fn DESKTOP_NAMES(&self) -> xcb::Atom {
		self.ewmh._NET_DESKTOP_NAMES
	}

	#[inline(always)]
	pub fn ACTIVE_WINDOW(&self) -> xcb::Atom {
		self.ewmh._NET_ACTIVE_WINDOW
	}

	#[inline(always)]
	pub fn WORKAREA(&self) -> xcb::Atom {
		self.ewmh._NET_WORKAREA
	}

	#[inline(always)]
	pub fn SUPPORTING_WM_CHECK(&self) -> xcb::Atom {
		self.ewmh._NET_SUPPORTING_WM_CHECK
	}

	#[inline(always)]
	pub fn VIRTUAL_ROOTS(&self) -> xcb::Atom {
		self.ewmh._NET_VIRTUAL_ROOTS
	}

	#[inline(always)]
	pub fn DESKTOP_LAYOUT(&self) -> xcb::Atom {
		self.ewmh._NET_DESKTOP_LAYOUT
	}

	#[inline(always)]
	pub fn SHOWING_DESKTOP(&self) -> xcb::Atom {
		self.ewmh._NET_SHOWING_DESKTOP
	}

	#[inline(always)]
	pub fn CLOSE_WINDOW(&self) -> xcb::Atom {
		self.ewmh._NET_CLOSE_WINDOW
	}

	#[inline(always)]
	pub fn MOVERESIZE_WINDOW(&self) -> xcb::Atom {
		self.ewmh._NET_MOVERESIZE_WINDOW
	}

	#[inline(always)]
	pub fn WM_MOVERESIZE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_MOVERESIZE
	}

	#[inline(always)]
	pub fn RESTACK_WINDOW(&self) -> xcb::Atom {
		self.ewmh._NET_RESTACK_WINDOW
	}

	#[inline(always)]
	pub fn REQUEST_FRAME_EXTENTS(&self) -> xcb::Atom {
		self.ewmh._NET_REQUEST_FRAME_EXTENTS
	}

	#[inline(always)]
	pub fn WM_NAME(&self) -> xcb::Atom {
		self.ewmh._NET_WM_NAME
	}

	#[inline(always)]
	pub fn WM_VISIBLE_NAME(&self) -> xcb::Atom {
		self.ewmh._NET_WM_VISIBLE_NAME
	}

	#[inline(always)]
	pub fn WM_ICON_NAME(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ICON_NAME
	}

	#[inline(always)]
	pub fn WM_VISIBLE_ICON_NAME(&self) -> xcb::Atom {
		self.ewmh._NET_WM_VISIBLE_ICON_NAME
	}

	#[inline(always)]
	pub fn WM_DESKTOP(&self) -> xcb::Atom {
		self.ewmh._NET_WM_DESKTOP
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE
	}

	#[inline(always)]
	pub fn WM_STATE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE
	}

	#[inline(always)]
	pub fn WM_ALLOWED_ACTIONS(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ALLOWED_ACTIONS
	}

	#[inline(always)]
	pub fn WM_STRUT(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STRUT
	}

	#[inline(always)]
	pub fn WM_STRUT_PARTIAL(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STRUT_PARTIAL
	}

	#[inline(always)]
	pub fn WM_ICON_GEOMETRY(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ICON_GEOMETRY
	}

	#[inline(always)]
	pub fn WM_ICON(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ICON
	}

	#[inline(always)]
	pub fn WM_PID(&self) -> xcb::Atom {
		self.ewmh._NET_WM_PID
	}

	#[inline(always)]
	pub fn WM_HANDLED_ICONS(&self) -> xcb::Atom {
		self.ewmh._NET_WM_HANDLED_ICONS
	}

	#[inline(always)]
	pub fn WM_USER_TIME(&self) -> xcb::Atom {
		self.ewmh._NET_WM_USER_TIME
	}

	#[inline(always)]
	pub fn WM_USER_TIME_WINDOW(&self) -> xcb::Atom {
		self.ewmh._NET_WM_USER_TIME_WINDOW
	}

	#[inline(always)]
	pub fn FRAME_EXTENTS(&self) -> xcb::Atom {
		self.ewmh._NET_FRAME_EXTENTS
	}

	#[inline(always)]
	pub fn WM_PING(&self) -> xcb::Atom {
		self.ewmh._NET_WM_PING
	}

	#[inline(always)]
	pub fn WM_SYNC_REQUEST(&self) -> xcb::Atom {
		self.ewmh._NET_WM_SYNC_REQUEST
	}

	#[inline(always)]
	pub fn WM_SYNC_REQUEST_COUNTER(&self) -> xcb::Atom {
		self.ewmh._NET_WM_SYNC_REQUEST_COUNTER
	}

	#[inline(always)]
	pub fn WM_FULLSCREEN_MONITORS(&self) -> xcb::Atom {
		self.ewmh._NET_WM_FULLSCREEN_MONITORS
	}

	#[inline(always)]
	pub fn WM_FULL_PLACEMENT(&self) -> xcb::Atom {
		self.ewmh._NET_WM_FULL_PLACEMENT
	}

	#[inline(always)]
	pub fn WM_PROTOCOLS(&self) -> xcb::Atom {
		self.ewmh.WM_PROTOCOLS
	}

	#[inline(always)]
	pub fn MANAGER(&self) -> xcb::Atom {
		self.ewmh.MANAGER
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_DESKTOP(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_DESKTOP
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_DOCK(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_DOCK
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_TOOLBAR(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_TOOLBAR
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_MENU(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_MENU
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_UTILITY(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_UTILITY
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_SPLASH(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_SPLASH
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_DIALOG(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_DIALOG
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_DROPDOWN_MENU(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_DROPDOWN_MENU
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_POPUP_MENU(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_POPUP_MENU
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_TOOLTIP(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_TOOLTIP
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_NOTIFICATION(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_NOTIFICATION
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_COMBO(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_COMBO
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_DND(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_DND
	}

	#[inline(always)]
	pub fn WM_WINDOW_TYPE_NORMAL(&self) -> xcb::Atom {
		self.ewmh._NET_WM_WINDOW_TYPE_NORMAL
	}

	#[inline(always)]
	pub fn WM_STATE_MODAL(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_MODAL
	}

	#[inline(always)]
	pub fn WM_STATE_STICKY(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_STICKY
	}

	#[inline(always)]
	pub fn WM_STATE_MAXIMIZED_VERT(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_MAXIMIZED_VERT
	}

	#[inline(always)]
	pub fn WM_STATE_MAXIMIZED_HORZ(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_MAXIMIZED_HORZ
	}

	#[inline(always)]
	pub fn WM_STATE_SHADED(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_SHADED
	}

	#[inline(always)]
	pub fn WM_STATE_SKIP_TASKBAR(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_SKIP_TASKBAR
	}

	#[inline(always)]
	pub fn WM_STATE_SKIP_PAGER(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_SKIP_PAGER
	}

	#[inline(always)]
	pub fn WM_STATE_HIDDEN(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_HIDDEN
	}

	#[inline(always)]
	pub fn WM_STATE_FULLSCREEN(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_FULLSCREEN
	}

	#[inline(always)]
	pub fn WM_STATE_ABOVE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_ABOVE
	}

	#[inline(always)]
	pub fn WM_STATE_BELOW(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_BELOW
	}

	#[inline(always)]
	pub fn WM_STATE_DEMANDS_ATTENTION(&self) -> xcb::Atom {
		self.ewmh._NET_WM_STATE_DEMANDS_ATTENTION
	}

	#[inline(always)]
	pub fn WM_ACTION_MOVE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_MOVE
	}

	#[inline(always)]
	pub fn WM_ACTION_RESIZE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_RESIZE
	}

	#[inline(always)]
	pub fn WM_ACTION_MINIMIZE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_MINIMIZE
	}

	#[inline(always)]
	pub fn WM_ACTION_SHADE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_SHADE
	}

	#[inline(always)]
	pub fn WM_ACTION_STICK(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_STICK
	}

	#[inline(always)]
	pub fn WM_ACTION_MAXIMIZE_HORZ(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_MAXIMIZE_HORZ
	}

	#[inline(always)]
	pub fn WM_ACTION_MAXIMIZE_VERT(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_MAXIMIZE_VERT
	}

	#[inline(always)]
	pub fn WM_ACTION_FULLSCREEN(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_FULLSCREEN
	}

	#[inline(always)]
	pub fn WM_ACTION_CHANGE_DESKTOP(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_CHANGE_DESKTOP
	}

	#[inline(always)]
	pub fn WM_ACTION_CLOSE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_CLOSE
	}

	#[inline(always)]
	pub fn WM_ACTION_ABOVE(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_ABOVE
	}

	#[inline(always)]
	pub fn WM_ACTION_BELOW(&self) -> xcb::Atom {
		self.ewmh._NET_WM_ACTION_BELOW
	}
}

impl Deref for Connection {
	type Target = xcb::Connection;

	fn deref(&self) -> &Self::Target {
		&self.xcb
	}
}

impl DerefMut for Connection {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.xcb
	}
}

impl Drop for Connection {
	fn drop(&mut self) {
		unsafe {
			xcb_ewmh_connection_wipe(&mut self.ewmh)
		}
	}
}

pub fn send_client_message<'a>(c: &'a xcb::Connection, window: xcb::Window, dest: xcb::Window, atom: xcb::Atom, data: &[u32]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_send_client_message(c.get_raw_conn(), window, dest, atom,
			data.len() as u32, data.as_ptr()))
}

pub fn request_close_window(c: &Connection, screen: i32, window: xcb::Window, timestamp: xcb::Timestamp, source_indication: ClientSourceType) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_close_window(c.get_raw_conn(), screen as c_int, window, timestamp, source_indication))
}

pub fn request_move_resize_window(c: &Connection, screen: i32, window: xcb::Window, gravity: xcb::Gravity, source_indication: ClientSourceType, flags: MoveResizeWindowFlags, x: u32, y: u32, width: u32, height: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_moveresize_window(c.get_raw_conn(), screen as c_int, window,
			gravity, source_indication, flags, x, y, width, height))
}

pub fn send_wm_ping(c: &Connection, window: xcb::Window, timestamp: xcb::Timestamp) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_send_wm_ping(c.get_raw_conn(), window, timestamp))
}

define!(cookie GetSupportedCookie through Connection with xcb_ewmh_get_supported_reply => GetSupportedReply);
define!(reply GetSupportedReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetSupportedReply {
	pub fn atoms(&self) -> &[xcb::Atom] {
		unsafe {
			slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize)
		}
	}
}

pub fn set_supported<'a>(c: &'a Connection, screen: i32, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_supported(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr()))
}

pub fn set_supported_checked<'a>(c: &'a Connection, screen: i32, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_supported_checked(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr()))
}

pub fn get_supported(c: &Connection, screen: i32) -> GetSupportedCookie {
	property!(checked GetSupportedCookie -> c,
		xcb_ewmh_get_supported(c.get_raw_conn(), screen as c_int))
}

pub fn get_supported_unchecked(c: &Connection, screen: i32) -> GetSupportedCookie {
	property!(unchecked GetSupportedCookie -> c,
		xcb_ewmh_get_supported_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetClientListCookie through Connection with xcb_ewmh_get_client_list_reply => GetClientListReply);
define!(reply GetClientListReply for xcb_ewmh_get_windows_reply_t with xcb_ewmh_get_windows_reply_wipe);

impl GetClientListReply {
	pub fn windows(&self) -> &[xcb::Window] {
		unsafe {
			slice::from_raw_parts(self.0.windows as *mut _, self.0.windows_len as usize)
		}
	}
}

pub fn set_client_list<'a>(c: &'a Connection, screen: i32, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_client_list(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn set_client_list_checked<'a>(c: &'a Connection, screen: i32, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_client_list_checked(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn get_client_list(c: &Connection, screen: i32) -> GetClientListCookie {
	property!(checked GetClientListCookie -> c,
		xcb_ewmh_get_client_list(c.get_raw_conn(), screen as c_int))
}

pub fn get_client_list_unchecked(c: &Connection, screen: i32) -> GetClientListCookie {
	property!(unchecked GetClientListCookie -> c,
		xcb_ewmh_get_client_list(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetClientListStackingCookie through Connection with xcb_ewmh_get_client_list_stacking_reply => GetClientListStackingReply);
define!(reply GetClientListStackingReply for xcb_ewmh_get_windows_reply_t with xcb_ewmh_get_windows_reply_wipe);

impl GetClientListStackingReply {
	pub fn windows(&self) -> &[xcb::Window] {
		unsafe {
			slice::from_raw_parts(self.0.windows as *mut _, self.0.windows_len as usize)
		}
	}
}

pub fn set_client_list_stacking<'a>(c: &'a Connection, screen: i32, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_client_list_stacking(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn set_client_list_stacking_checked<'a>(c: &'a Connection, screen: i32, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_client_list_stacking_checked(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr() as *const _))
}

pub fn get_client_list_stacking(c: &Connection, screen: i32) -> GetClientListStackingCookie {
	property!(checked GetClientListStackingCookie -> c,
		xcb_ewmh_get_client_list_stacking(c.get_raw_conn(), screen as c_int))
}

pub fn get_client_list_stacking_unchecked(c: &Connection, screen: i32) -> GetClientListStackingCookie {
	property!(unchecked GetClientListStackingCookie -> c,
		xcb_ewmh_get_client_list_stacking(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetNumberOfDesktopsCookie through Connection with xcb_ewmh_get_number_of_desktops_reply as u32);

pub fn set_number_of_desktops(c: &Connection, screen: i32, number: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_number_of_desktops(c.get_raw_conn(), screen as c_int, number))
}

pub fn set_number_of_desktops_checked(c: &Connection, screen: i32, number: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_number_of_desktops_checked(c.get_raw_conn(), screen as c_int, number))
}

pub fn get_number_of_desktops(c: &Connection, screen: i32) -> GetNumberOfDesktopsCookie {
	property!(checked GetNumberOfDesktopsCookie -> c,
		xcb_ewmh_get_number_of_desktops(c.get_raw_conn(), screen as c_int))
}

pub fn get_number_of_desktops_unchecked(c: &Connection, screen: i32) -> GetNumberOfDesktopsCookie {
	property!(unchecked GetNumberOfDesktopsCookie -> c,
		xcb_ewmh_get_number_of_desktops_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopGeometryCookie through Connection with xcb_ewmh_get_desktop_geometry_reply as (u32, u32));

pub fn set_desktop_geometry(c: &Connection, screen: i32, width: u32, height: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_desktop_geometry(c.get_raw_conn(), screen as c_int, width, height))
}

pub fn set_desktop_geometry_checked(c: &Connection, screen: i32, width: u32, height: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_desktop_geometry_checked(c.get_raw_conn(), screen as c_int, width, height))
}

pub fn request_change_desktop_geometry(c: &Connection, screen: i32, width: u32, height: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_change_desktop_geometry(c.get_raw_conn(), screen as c_int, width, height))
}

pub fn get_desktop_geometry(c: &Connection, screen: i32) -> GetDesktopGeometryCookie {
	property!(checked GetDesktopGeometryCookie -> c,
		xcb_ewmh_get_desktop_geometry(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_geometry_unchecked(c: &Connection, screen: i32) -> GetDesktopGeometryCookie {
	property!(unchecked GetDesktopGeometryCookie -> c,
		xcb_ewmh_get_desktop_geometry_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopViewportCookie through Connection with xcb_ewmh_get_desktop_viewport_reply => GetDesktopViewportReply);
define!(reply GetDesktopViewportReply for xcb_ewmh_get_desktop_viewport_reply_t with xcb_ewmh_get_desktop_viewport_reply_wipe);

impl GetDesktopViewportReply {
	pub fn desktop_viewports(&self) -> &[Coordinates] {
		unsafe {
			slice::from_raw_parts(self.0.desktop_viewport as *mut _, self.0.desktop_viewport_len as usize)
		}
	}
}

pub fn set_desktop_viewport<'a>(c: &'a Connection, screen: i32, list: &[Coordinates]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_desktop_viewport(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr()))
}

pub fn set_desktop_viewport_checked<'a>(c: &'a Connection, screen: i32, list: &[Coordinates]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_desktop_viewport_checked(c.get_raw_conn(), screen as c_int,
			list.len() as u32, list.as_ptr()))
}

pub fn get_desktop_viewport(c: &Connection, screen: i32) -> GetDesktopViewportCookie {
	property!(checked GetDesktopViewportCookie -> c,
		xcb_ewmh_get_desktop_viewport(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_viewport_unchecked(c: &Connection, screen: i32) -> GetDesktopViewportCookie {
	property!(unchecked GetDesktopViewportCookie -> c,
		xcb_ewmh_get_desktop_viewport_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetCurrentDesktopCookie through Connection with xcb_ewmh_get_current_desktop_reply as u32);

pub fn set_current_desktop(c: &Connection, screen: i32, current_desktop: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_current_desktop(c.get_raw_conn(), screen as c_int, current_desktop))
}

pub fn set_current_desktop_checked(c: &Connection, screen: i32, current_desktop: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_current_desktop_checked(c.get_raw_conn(), screen as c_int, current_desktop))
}

pub fn request_change_current_desktop(c: &Connection, screen: i32, current_desktop: u32, timestamp: xcb::Timestamp) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_change_current_desktop(c.get_raw_conn(), screen as c_int, current_desktop, timestamp))
}

pub fn get_current_desktop(c: &Connection, screen: i32) -> GetCurrentDesktopCookie {
	property!(checked GetCurrentDesktopCookie -> c,
		xcb_ewmh_get_current_desktop(c.get_raw_conn(), screen as c_int))
}

pub fn get_current_desktop_unchecked(c: &Connection, screen: i32) -> GetCurrentDesktopCookie {
	property!(unchecked GetCurrentDesktopCookie -> c,
		xcb_ewmh_get_current_desktop_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopNamesCookie through Connection with xcb_ewmh_get_desktop_names_reply => GetDesktopNamesReply);
define!(reply GetDesktopNamesReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

impl GetDesktopNamesReply {
	pub fn strings(&self) -> Vec<&str> {
		utf8::into(self.0.strings, self.0.strings_len)
	}
}

pub fn set_desktop_names<'a, T: IntoIterator<Item=&'a str>>(c: &Connection, screen: i32, list: T) -> xcb::VoidCookie {
	let value = utf8::from(list);

	void!(unchecked -> c,
		xcb_ewmh_set_desktop_names(c.get_raw_conn(), screen, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn set_desktop_names_checked<'a, T: IntoIterator<Item=&'a str>>(c: &Connection, screen: i32, list: T) -> xcb::VoidCookie {
	let value = utf8::from(list);

	void!(checked -> c,
		xcb_ewmh_set_desktop_names_checked(c.get_raw_conn(), screen, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn get_desktop_names(c: &Connection, screen: i32) -> GetDesktopNamesCookie {
	property!(checked GetDesktopNamesCookie -> c,
		xcb_ewmh_get_desktop_names(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_names_unchecked(c: &Connection, screen: i32) -> GetDesktopNamesCookie {
	property!(unchecked GetDesktopNamesCookie -> c,
		xcb_ewmh_get_desktop_names_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetActiveWindowCookie through Connection with xcb_ewmh_get_active_window_reply as xcb::Window);

pub fn set_active_window(c: &Connection, screen: i32, window: xcb::Window) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_active_window(c.get_raw_conn(), screen as c_int, window))
}

pub fn set_active_window_checked(c: &Connection, screen: i32, window: xcb::Window) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_active_window_checked(c.get_raw_conn(), screen as c_int, window))
}

pub fn request_change_active_window(c: &Connection, screen: i32, window: xcb::Window, source_indication: ClientSourceType, timestamp: xcb::Timestamp, current: xcb::Window) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_change_active_window(c.get_raw_conn(), screen as c_int, window,
			source_indication, timestamp, current))
}

pub fn get_active_window(c: &Connection, screen: i32) -> GetActiveWindowCookie {
	property!(checked GetActiveWindowCookie -> c,
		xcb_ewmh_get_active_window(c.get_raw_conn(), screen as c_int))
}

pub fn get_active_window_unchecked(c: &Connection, screen: i32) -> GetActiveWindowCookie {
	property!(unchecked GetActiveWindowCookie -> c,
		xcb_ewmh_get_active_window_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetWorkAreaCookie through Connection with xcb_ewmh_get_workarea_reply => GetWorkAreaReply);
define!(reply GetWorkAreaReply for xcb_ewmh_get_workarea_reply_t with xcb_ewmh_get_workarea_reply_wipe);

impl GetWorkAreaReply {
	pub fn work_area(&self) -> &[Geometry] {
		unsafe {
			slice::from_raw_parts(self.0.workarea, self.0.workarea_len as usize)
		}
	}
}

pub fn set_work_area<'a>(c: &'a Connection, screen: i32, list: &[Geometry]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_workarea(c.get_raw_conn(), screen as c_int, list.len() as u32, list.as_ptr()))
}

pub fn set_work_area_checked<'a>(c: &'a Connection, screen: i32, list: &[Geometry]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_workarea_checked(c.get_raw_conn(), screen as c_int, list.len() as u32, list.as_ptr()))
}

pub fn get_work_area(c: &Connection, screen: i32) -> GetWorkAreaCookie {
	property!(checked GetWorkAreaCookie -> c,
		xcb_ewmh_get_workarea(c.get_raw_conn(), screen as c_int))
}

pub fn get_work_area_unchecked(c: &Connection, screen: i32) -> GetWorkAreaCookie {
	property!(unchecked GetWorkAreaCookie -> c,
		xcb_ewmh_get_workarea_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetSupportingWmCheckCookie through Connection with xcb_ewmh_get_supporting_wm_check_reply as xcb::Window);

pub fn set_supporting_wm_check(c: &Connection, parent: xcb::Window, child: xcb::Window) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_supporting_wm_check(c.get_raw_conn(), parent, child))
}

pub fn set_supporting_wm_check_checked(c: &Connection, parent: xcb::Window, child: xcb::Window) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_supporting_wm_check_checked(c.get_raw_conn(), parent, child))
}

pub fn get_supporting_wm_check(c: &Connection, window: xcb::Window) -> GetSupportingWmCheckCookie {
	property!(checked GetSupportingWmCheckCookie -> c,
		xcb_ewmh_get_supporting_wm_check(c.get_raw_conn(), window))
}

pub fn get_supporting_wm_check_unchecked(c: &Connection, window: xcb::Window) -> GetSupportingWmCheckCookie {
	property!(unchecked GetSupportingWmCheckCookie -> c,
		xcb_ewmh_get_supporting_wm_check_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetVirtualRootsCookie through Connection with xcb_ewmh_get_virtual_roots_reply => GetVirtualRootsReply);
define!(reply GetVirtualRootsReply for xcb_ewmh_get_windows_reply_t with xcb_ewmh_get_windows_reply_wipe);

pub fn set_virtual_roots<'a>(c: &'a Connection, screen: i32, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_virtual_roots(c.get_raw_conn(), screen as c_int, list.len() as u32, list.as_ptr()))
}

pub fn set_virtual_roots_checked<'a>(c: &'a Connection, screen: i32, list: &[xcb::Window]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_virtual_roots_checked(c.get_raw_conn(), screen as c_int, list.len() as u32, list.as_ptr()))
}

pub fn get_virtual_roots(c: &Connection, screen: i32) -> GetVirtualRootsCookie {
	property!(checked GetVirtualRootsCookie -> c,
		xcb_ewmh_get_virtual_roots(c.get_raw_conn(), screen as c_int))
}

pub fn get_virtual_roots_unchecked(c: &Connection, screen: i32) -> GetVirtualRootsCookie {
	property!(unchecked GetVirtualRootsCookie -> c,
		xcb_ewmh_get_virtual_roots_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetDesktopLayoutCookie through Connection with xcb_ewmh_get_desktop_layout_reply => GetDesktopLayoutReply);
define!(reply GetDesktopLayoutReply for xcb_ewmh_get_desktop_layout_reply_t);

impl GetDesktopLayoutReply {
	pub fn orientation(&self) -> DesktopLayoutOrientation {
		self.0.orientation
	}

	pub fn columns(&self) -> u32 {
		self.0.columns
	}

	pub fn rows(&self) -> u32 {
		self.0.rows
	}

	pub fn starting_corner(&self) -> DesktopLayoutStartingCorner {
		self.0.starting_corner
	}
}

pub fn set_desktop_layout(c: &Connection, screen: i32, orientation: DesktopLayoutOrientation, columns: u32, rows: u32, starting_corner: DesktopLayoutStartingCorner) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_desktop_layout(c.get_raw_conn(), screen as c_int, orientation, columns, rows, starting_corner))
}

pub fn set_desktop_layout_checked(c: &Connection, screen: i32, orientation: DesktopLayoutOrientation, columns: u32, rows: u32, starting_corner: DesktopLayoutStartingCorner) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_desktop_layout_checked(c.get_raw_conn(), screen as c_int, orientation, columns, rows, starting_corner))
}

pub fn get_desktop_layout(c: &Connection, screen: i32) -> GetDesktopLayoutCookie {
	property!(checked GetDesktopLayoutCookie -> c,
		xcb_ewmh_get_desktop_layout(c.get_raw_conn(), screen as c_int))
}

pub fn get_desktop_layout_unchecked(c: &Connection, screen: i32) -> GetDesktopLayoutCookie {
	property!(unchecked GetDesktopLayoutCookie -> c,
		xcb_ewmh_get_desktop_layout_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetShowingDesktopCookie through Connection with xcb_ewmh_get_showing_desktop_reply as u32);

pub fn set_showing_desktop(c: &Connection, screen: i32, desktop: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_showing_desktop(c.get_raw_conn(), screen as c_int, desktop))
}

pub fn set_showing_desktop_checked(c: &Connection, screen: i32, desktop: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_showing_desktop_checked(c.get_raw_conn(), screen as c_int, desktop))
}

pub fn get_showing_desktop(c: &Connection, screen: i32) -> GetShowingDesktopCookie {
	property!(checked GetShowingDesktopCookie -> c,
		xcb_ewmh_get_showing_desktop(c.get_raw_conn(), screen as c_int))
}

pub fn get_showing_desktop_unchecked(c: &Connection, screen: i32) -> GetShowingDesktopCookie {
	property!(unchecked GetShowingDesktopCookie -> c,
		xcb_ewmh_get_showing_desktop_unchecked(c.get_raw_conn(), screen as c_int))
}

define!(cookie GetWmNameCookie through Connection with xcb_ewmh_get_wm_name_reply => GetWmNameReply);
define!(reply GetWmNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

impl GetWmNameReply {
	pub fn string(&self) -> &str {
		utf8::into(self.0.strings, self.0.strings_len)
			.get(0)
			.unwrap_or(&"")
	}
}

pub fn set_wm_name<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = name.as_ref();

	void!(unchecked -> c,
		xcb_ewmh_set_wm_name(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn set_wm_name_checked<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = name.as_ref();

	void!(checked -> c,
		xcb_ewmh_set_wm_name_checked(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn get_wm_name(c: &Connection, window: xcb::Window) -> GetWmNameCookie {
	property!(checked GetWmNameCookie -> c,
		xcb_ewmh_get_wm_name(c.get_raw_conn(), window))
}

pub fn get_wm_name_unchecked(c: &Connection, window: xcb::Window) -> GetWmNameCookie {
	property!(unchecked GetWmNameCookie -> c,
		xcb_ewmh_get_wm_name_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmVisibleNameCookie through Connection with xcb_ewmh_get_wm_visible_name_reply => GetWmVisibleNameReply);
define!(reply GetWmVisibleNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

pub fn set_wm_visible_name<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = utf8::from(vec![name.as_ref()]);

	void!(unchecked -> c,
		xcb_ewmh_set_wm_visible_name(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn set_wm_visible_name_checked<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = utf8::from(vec![name.as_ref()]);

	void!(checked -> c,
		xcb_ewmh_set_wm_visible_name_checked(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn get_wm_visible_name(c: &Connection, window: xcb::Window) -> GetWmVisibleNameCookie {
	property!(checked GetWmVisibleNameCookie -> c,
		xcb_ewmh_get_wm_visible_name(c.get_raw_conn(), window))
}

pub fn get_wm_visible_name_unchecked(c: &Connection, window: xcb::Window) -> GetWmVisibleNameCookie {
	property!(unchecked GetWmVisibleNameCookie -> c,
		xcb_ewmh_get_wm_visible_name_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmIconNameCookie through Connection with xcb_ewmh_get_wm_icon_name_reply => GetWmIconNameReply);
define!(reply GetWmIconNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

pub fn set_wm_icon_name<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = utf8::from(vec![name.as_ref()]);

	void!(unchecked -> c,
		xcb_ewmh_set_wm_icon_name(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn set_wm_icon_name_checked<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = utf8::from(vec![name.as_ref()]);

	void!(checked -> c,
		xcb_ewmh_set_wm_icon_name_checked(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn get_wm_icon_name(c: &Connection, window: xcb::Window) -> GetWmIconNameCookie {
	property!(checked GetWmIconNameCookie -> c,
		xcb_ewmh_get_wm_icon_name(c.get_raw_conn(), window))
}

pub fn get_wm_icon_name_unchecked(c: &Connection, window: xcb::Window) -> GetWmIconNameCookie {
	property!(unchecked GetWmIconNameCookie -> c,
		xcb_ewmh_get_wm_icon_name_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmVisibleIconNameCookie through Connection with xcb_ewmh_get_wm_visible_icon_name_reply => GetWmVisibleIconNameReply);
define!(reply GetWmVisibleIconNameReply for xcb_ewmh_get_utf8_strings_reply_t with xcb_ewmh_get_utf8_strings_reply_wipe);

pub fn set_wm_visible_icon_name<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = utf8::from(vec![name.as_ref()]);

	void!(unchecked -> c,
		xcb_ewmh_set_wm_visible_icon_name(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn set_wm_visible_icon_name_checked<T: AsRef<str>>(c: &Connection, window: xcb::Window, name: T) -> xcb::VoidCookie {
	let value = utf8::from(vec![name.as_ref()]);

	void!(checked -> c,
		xcb_ewmh_set_wm_visible_icon_name_checked(c.get_raw_conn(), window, value.len() as u32, value.as_ptr() as *mut _))
}

pub fn get_wm_visible_icon_name(c: &Connection, window: xcb::Window) -> GetWmVisibleIconNameCookie {
	property!(checked GetWmVisibleIconNameCookie -> c,
		xcb_ewmh_get_wm_visible_icon_name(c.get_raw_conn(), window))
}

pub fn get_wm_visible_icon_name_unchecked(c: &Connection, window: xcb::Window) -> GetWmVisibleIconNameCookie {
	property!(unchecked GetWmVisibleIconNameCookie -> c,
		xcb_ewmh_get_wm_visible_icon_name_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmDesktopCookie through Connection with xcb_ewmh_get_wm_desktop_reply as u32);

pub fn set_wm_desktop(c: &Connection, window: xcb::Window, number: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_desktop(c.get_raw_conn(), window, number))
}

pub fn set_wm_desktop_checked(c: &Connection, window: xcb::Window, number: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_desktop_checked(c.get_raw_conn(), window, number))
}

pub fn request_change_wm_desktop(c: &Connection, screen: i32, window: xcb::Window, desktop: u32, source_indication: ClientSourceType) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_change_wm_desktop(c.get_raw_conn(), screen as c_int, window,
			desktop, source_indication))
}

pub fn get_wm_desktop(c: &Connection, window: xcb::Window) -> GetWmDesktopCookie {
	property!(checked GetWmDesktopCookie -> c,
		xcb_ewmh_get_wm_desktop(c.get_raw_conn(), window))
}

pub fn get_wm_desktop_unchecked(c: &Connection, window: xcb::Window) -> GetWmDesktopCookie {
	property!(unchecked GetWmDesktopCookie -> c,
		xcb_ewmh_get_wm_desktop_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmWindowTypeCookie through Connection with xcb_ewmh_get_wm_window_type_reply => GetWmWindowTypeReply);
define!(reply GetWmWindowTypeReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetWmWindowTypeReply {
	pub fn atoms(&self) -> &[xcb::Atom] {
		unsafe {
			slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize)
		}
	}
}

pub fn set_wm_window_type<'a>(c: &'a Connection, window: xcb::Window, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_window_type(c.get_raw_conn(), window, list.len() as u32, list.as_ptr()))
}

pub fn set_wm_window_type_checked<'a>(c: &'a Connection, window: xcb::Window, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_wm_window_type_checked(c.get_raw_conn(), window, list.len() as u32, list.as_ptr()))
}

pub fn get_wm_window_type(c: &Connection, window: xcb::Window) -> GetWmWindowTypeCookie {
	property!(checked GetWmWindowTypeCookie -> c,
		xcb_ewmh_get_wm_window_type(c.get_raw_conn(), window))
}

pub fn get_wm_window_type_unchecked(c: &Connection, window: xcb::Window) -> GetWmWindowTypeCookie {
	property!(unchecked GetWmWindowTypeCookie -> c,
		xcb_ewmh_get_wm_window_type_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmStateCookie through Connection with xcb_ewmh_get_wm_state_reply => GetWmStateReply);
define!(reply GetWmStateReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetWmStateReply {
	pub fn atoms(&self) -> &[xcb::Atom] {
		unsafe {
			slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize)
		}
	}
}

pub fn set_wm_state<'a>(c: &'a Connection, window: xcb::Window, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_state(c.get_raw_conn(), window, list.len() as u32, list.as_ptr()))
}

pub fn set_wm_state_checked<'a>(c: &'a Connection, window: xcb::Window, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_wm_state_checked(c.get_raw_conn(), window, list.len() as u32, list.as_ptr()))
}

pub fn request_change_wm_state(c: &Connection, screen: i32, window: xcb::Window, action: StateAction, first: xcb::Atom, second: xcb::Atom, source_indication: ClientSourceType) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_change_wm_state(c.get_raw_conn(), screen as c_int, window, action, first, second, source_indication))
}

pub fn get_wm_state(c: &Connection, window: xcb::Window) -> GetWmStateCookie {
	property!(checked GetWmStateCookie -> c,
		xcb_ewmh_get_wm_state(c.get_raw_conn(), window))
}

pub fn get_wm_state_unchecked(c: &Connection, window: xcb::Window) -> GetWmStateCookie {
	property!(unchecked GetWmStateCookie -> c,
		xcb_ewmh_get_wm_state_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmAllowedActionsCookie through Connection with xcb_ewmh_get_wm_allowed_actions_reply => GetWmAllowedActionsReply);
define!(reply GetWmAllowedActionsReply for xcb_ewmh_get_atoms_reply_t with xcb_ewmh_get_atoms_reply_wipe);

impl GetWmAllowedActionsReply {
	pub fn atoms(&self) -> &[xcb::Atom] {
		unsafe {
			slice::from_raw_parts(self.0.atoms as *mut _, self.0.atoms_len as usize)
		}
	}
}

pub fn set_wm_allowed_actions<'a>(c: &'a Connection, window: xcb::Window, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_allowed_actions(c.get_raw_conn(), window, list.len() as u32, list.as_ptr()))
}

pub fn set_wm_allowed_actions_checked<'a>(c: &'a Connection, window: xcb::Window, list: &[xcb::Atom]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_wm_allowed_actions_checked(c.get_raw_conn(), window, list.len() as u32, list.as_ptr()))
}

pub fn get_wm_allowed_actions(c: &Connection, window: xcb::Window) -> GetWmAllowedActionsCookie {
	property!(checked GetWmAllowedActionsCookie -> c,
		xcb_ewmh_get_wm_allowed_actions(c.get_raw_conn(), window))
}

pub fn get_wm_allowed_actions_unchecked(c: &Connection, window: xcb::Window) -> GetWmAllowedActionsCookie {
	property!(unchecked GetWmAllowedActionsCookie -> c,
		xcb_ewmh_get_wm_allowed_actions_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmStrutCookie through Connection with xcb_ewmh_get_wm_strut_reply as Extents);

pub fn set_wm_strut(c: &Connection, window: xcb::Window, left: u32, right: u32, top: u32, bottom: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_strut(c.get_raw_conn(), window, left, right, top, bottom))
}

pub fn set_wm_strut_checked(c: &Connection, window: xcb::Window, left: u32, right: u32, top: u32, bottom: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_strut_checked(c.get_raw_conn(), window, left, right, top, bottom))
}

pub fn get_wm_strut(c: &Connection, window: xcb::Window) -> GetWmStrutCookie {
	property!(checked GetWmStrutCookie -> c,
		xcb_ewmh_get_wm_strut(c.get_raw_conn(), window))
}

pub fn get_wm_strut_unchecked(c: &Connection, window: xcb::Window) -> GetWmStrutCookie {
	property!(unchecked GetWmStrutCookie -> c,
		xcb_ewmh_get_wm_strut(c.get_raw_conn(), window))
}

define!(cookie GetWmStrutPartialCookie through Connection with xcb_ewmh_get_wm_strut_partial_reply as StrutPartial);

pub fn set_wm_strut_partial(c: &Connection, window: xcb::Window, partial: StrutPartial) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_strut_partial(c.get_raw_conn(), window, partial))
}

pub fn set_wm_strut_partial_checked(c: &Connection, window: xcb::Window, partial: StrutPartial) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_strut_partial_checked(c.get_raw_conn(), window, partial))
}

pub fn get_wm_strut_partial(c: &Connection, window: xcb::Window) -> GetWmStrutPartialCookie {
	property!(checked GetWmStrutPartialCookie -> c,
		xcb_ewmh_get_wm_strut_partial(c.get_raw_conn(), window))
}

pub fn get_wm_strut_partial_unchecked(c: &Connection, window: xcb::Window) -> GetWmStrutPartialCookie {
	property!(unchecked GetWmStrutPartialCookie -> c,
		xcb_ewmh_get_wm_strut_partial(c.get_raw_conn(), window))
}

define!(cookie GetWmIconGeometryCookie through Connection with xcb_ewmh_get_wm_icon_geometry_reply as Geometry);

pub fn set_wm_icon_geometry(c: &Connection, window: xcb::Window, left: u32, right: u32, top: u32, bottom: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_icon_geometry(c.get_raw_conn(), window, left, right, top, bottom))
}

pub fn set_wm_icon_geometry_checked(c: &Connection, window: xcb::Window, left: u32, right: u32, top: u32, bottom: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_icon_geometry_checked(c.get_raw_conn(), window, left, right, top, bottom))
}

pub fn get_wm_icon_geometry(c: &Connection, window: xcb::Window) -> GetWmIconGeometryCookie {
	property!(checked GetWmIconGeometryCookie -> c,
		xcb_ewmh_get_wm_icon_geometry(c.get_raw_conn(), window))
}

pub fn get_wm_icon_geometry_unchecked(c: &Connection, window: xcb::Window) -> GetWmIconGeometryCookie {
	property!(unchecked GetWmIconGeometryCookie -> c,
		xcb_ewmh_get_wm_icon_geometry_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmIconCookie through Connection with xcb_ewmh_get_wm_icon_reply => GetWmIconReply);
define!(reply GetWmIconReply for xcb_ewmh_get_wm_icon_reply_t with xcb_ewmh_get_wm_icon_reply_wipe);

impl GetWmIconReply {
	pub fn len(&self) -> usize {
		unsafe {
			xcb_ewmh_get_wm_icon_length(&self.0) as usize
		}
	}

	pub fn icons(&self) -> WmIconIterator {
		unsafe {
			xcb_ewmh_get_wm_icon_iterator(&self.0)
		}
	}
}

pub fn set_wm_icon<'a>(c: &'a Connection, mode: u8, window: xcb::Window, data: &[u32]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_icon(c.get_raw_conn(), mode, window, data.len() as u32, data.as_ptr()))
}

pub fn set_wm_icon_checked<'a>(c: &'a Connection, mode: u8, window: xcb::Window, data: &[u32]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_set_wm_icon_checked(c.get_raw_conn(), mode, window, data.len() as u32, data.as_ptr()))
}

pub fn append_wm_icon<'a>(c: &'a Connection, window: xcb::Window, width: u32, height: u32, img: &[u32]) -> xcb::VoidCookie<'a> {
	void!(unchecked -> c,
		xcb_ewmh_append_wm_icon(c.get_raw_conn(), window, width, height, img.len() as u32, img.as_ptr()))
}

pub fn append_wm_icon_checked<'a>(c: &'a Connection, window: xcb::Window, width: u32, height: u32, img: &[u32]) -> xcb::VoidCookie<'a> {
	void!(checked -> c,
		xcb_ewmh_append_wm_icon_checked(c.get_raw_conn(), window, width, height, img.len() as u32, img.as_ptr()))
}

pub fn get_wm_icon(c: &Connection, window: xcb::Window) -> GetWmIconCookie {
	property!(checked GetWmIconCookie -> c,
		xcb_ewmh_get_wm_icon(c.get_raw_conn(), window))
}

pub fn get_wm_icon_unchecked(c: &Connection, window: xcb::Window) -> GetWmIconCookie {
	property!(unchecked GetWmIconCookie -> c,
		xcb_ewmh_get_wm_icon_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmPidCookie through Connection with xcb_ewmh_get_wm_pid_reply as u32);

pub fn set_wm_pid(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_pid(c.get_raw_conn(), window, pid))
}

pub fn set_wm_pid_checked(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_pid_checked(c.get_raw_conn(), window, pid))
}

pub fn get_wm_pid(c: &Connection, window: xcb::Window) -> GetWmPidCookie {
	property!(checked GetWmPidCookie -> c,
		xcb_ewmh_get_wm_pid(c.get_raw_conn(), window))
}

pub fn get_wm_pid_unchecked(c: &Connection, window: xcb::Window) -> GetWmPidCookie {
	property!(checked GetWmPidCookie -> c,
		xcb_ewmh_get_wm_pid_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmHandledIconsCookie through Connection with xcb_ewmh_get_wm_handled_icons_reply as u32);

pub fn set_wm_handled_icons(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_handled_icons(c.get_raw_conn(), window, pid))
}

pub fn set_wm_handled_icons_checked(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_handled_icons_checked(c.get_raw_conn(), window, pid))
}

pub fn get_wm_handled_icons(c: &Connection, window: xcb::Window) -> GetWmHandledIconsCookie {
	property!(checked GetWmHandledIconsCookie -> c,
		xcb_ewmh_get_wm_handled_icons(c.get_raw_conn(), window))
}

pub fn get_wm_handled_icons_unchecked(c: &Connection, window: xcb::Window) -> GetWmHandledIconsCookie {
	property!(checked GetWmHandledIconsCookie -> c,
		xcb_ewmh_get_wm_handled_icons_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmUserTimeCookie through Connection with xcb_ewmh_get_wm_user_time_reply as u32);

pub fn set_wm_user_time(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_user_time(c.get_raw_conn(), window, pid))
}

pub fn set_wm_user_time_checked(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_user_time_checked(c.get_raw_conn(), window, pid))
}

pub fn get_wm_user_time(c: &Connection, window: xcb::Window) -> GetWmUserTimeCookie {
	property!(checked GetWmUserTimeCookie -> c,
		xcb_ewmh_get_wm_user_time(c.get_raw_conn(), window))
}

pub fn get_wm_user_time_unchecked(c: &Connection, window: xcb::Window) -> GetWmUserTimeCookie {
	property!(checked GetWmUserTimeCookie -> c,
		xcb_ewmh_get_wm_user_time_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmUserTimeWindowCookie through Connection with xcb_ewmh_get_wm_user_time_window_reply as u32);

pub fn set_wm_user_time_window(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_user_time_window(c.get_raw_conn(), window, pid))
}

pub fn set_wm_user_time_window_checked(c: &Connection, window: xcb::Window, pid: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_user_time_window_checked(c.get_raw_conn(), window, pid))
}

pub fn get_wm_user_time_window(c: &Connection, window: xcb::Window) -> GetWmUserTimeWindowCookie {
	property!(checked GetWmUserTimeWindowCookie -> c,
		xcb_ewmh_get_wm_user_time_window(c.get_raw_conn(), window))
}

pub fn get_wm_user_time_window_unchecked(c: &Connection, window: xcb::Window) -> GetWmUserTimeWindowCookie {
	property!(checked GetWmUserTimeWindowCookie -> c,
		xcb_ewmh_get_wm_user_time_window_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetFrameExtentsCookie through Connection with xcb_ewmh_get_frame_extents_reply as Extents);

pub fn set_frame_extents(c: &Connection, window: xcb::Window, left: u32, right: u32, top: u32, bottom: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_frame_extents(c.get_raw_conn(), window, left, right, top, bottom))
}

pub fn set_frame_extents_checked(c: &Connection, window: xcb::Window, left: u32, right: u32, top: u32, bottom: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_frame_extents_checked(c.get_raw_conn(), window, left, right, top, bottom))
}

pub fn get_frame_extents(c: &Connection, window: xcb::Window) -> GetFrameExtentsCookie {
	property!(checked GetFrameExtentsCookie -> c,
		xcb_ewmh_get_frame_extents(c.get_raw_conn(), window))
}

pub fn get_frame_extents_unchecked(c: &Connection, window: xcb::Window) -> GetFrameExtentsCookie {
	property!(unchecked GetFrameExtentsCookie -> c,
		xcb_ewmh_get_frame_extents_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmSyncRequestCounterCookie through Connection with xcb_ewmh_get_wm_sync_request_counter_reply as u64);

pub fn set_wm_sync_request_counter(c: &Connection, window: xcb::Window, atom: xcb::Atom, low: u32, high: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_sync_request_counter(c.get_raw_conn(), window, atom, low, high))
}

pub fn set_wm_sync_request_counter_checked(c: &Connection, window: xcb::Window, atom: xcb::Atom, low: u32, high: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_sync_request_counter_checked(c.get_raw_conn(), window, atom, low, high))
}

pub fn send_wm_sync_request(c: &Connection, window: xcb::Window, wm_protocols: xcb::Atom, wm_sync_request: xcb::Atom, timestamp: xcb::Timestamp, counter: u64) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_send_wm_sync_rqeuest(c.get_raw_conn(), window, wm_protocols, wm_sync_request, timestamp, counter))
}

pub fn get_wm_sync_request_counter(c: &Connection, window: xcb::Window) -> GetWmSyncRequestCounterCookie {
	property!(checked GetWmSyncRequestCounterCookie -> c,
		xcb_ewmh_get_wm_sync_request_counter(c.get_raw_conn(), window))
}

pub fn get_wm_sync_request_counter_unchecked(c: &Connection, window: xcb::Window) -> GetWmSyncRequestCounterCookie {
	property!(unchecked GetWmSyncRequestCounterCookie -> c,
		xcb_ewmh_get_wm_sync_request_counter_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmFullScreenMonitorsCookie through Connection with xcb_ewmh_get_wm_fullscreen_monitors_reply as WmFullScreenMonitors);

pub fn set_wm_full_screen_monitors(c: &Connection, window: xcb::Window, top: u32, bottom: u32, left: u32, right: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_fullscreen_monitors(c.get_raw_conn(), window, top, bottom, left, right))
}

pub fn set_wm_full_screen_monitors_checked(c: &Connection, window: xcb::Window, top: u32, bottom: u32, left: u32, right: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_fullscreen_monitors_checked(c.get_raw_conn(), window, top, bottom, left, right))
}

pub fn request_change_wm_full_screen_monitors(c: &Connection, screen: i32, window: xcb::Window, top: u32, bottom: u32, left: u32, right: u32, source_indication: ClientSourceType) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_request_change_wm_fullscreen_monitors(c.get_raw_conn(), screen as c_int, window, top, bottom, left, right, source_indication))
}

pub fn get_wm_full_screen_monitors(c: &Connection, window: xcb::Window) -> GetWmFullScreenMonitorsCookie {
	property!(checked GetWmFullScreenMonitorsCookie -> c,
		xcb_ewmh_get_wm_fullscreen_monitors(c.get_raw_conn(), window))
}

pub fn get_wm_full_screen_monitors_unchecked(c: &Connection, window: xcb::Window) -> GetWmFullScreenMonitorsCookie {
	property!(unchecked GetWmFullScreenMonitorsCookie -> c,
		xcb_ewmh_get_wm_fullscreen_monitors_unchecked(c.get_raw_conn(), window))
}

define!(cookie GetWmCmOwnerCookie(xcb_get_selection_owner_cookie_t) through Connection with xcb_ewmh_get_wm_cm_owner_reply as xcb::Window);

pub fn set_wm_cm_owner(c: &Connection, screen: i32, owner: xcb::Window, timestamp: xcb::Timestamp, first: u32, second: u32) -> xcb::VoidCookie {
	void!(unchecked -> c,
		xcb_ewmh_set_wm_cm_owner(c.get_raw_conn(), screen as c_int, owner, timestamp, first, second))
}

pub fn set_wm_cm_owner_checked(c: &Connection, screen: i32, owner: xcb::Window, timestamp: xcb::Timestamp, first: u32, second: u32) -> xcb::VoidCookie {
	void!(checked -> c,
		xcb_ewmh_set_wm_cm_owner_checked(c.get_raw_conn(), screen as c_int, owner, timestamp, first, second))
}

pub fn get_wm_cm_owner(c: &Connection, screen: i32) -> GetWmCmOwnerCookie {
	unsafe {
		GetWmCmOwnerCookie {
			conn:    c,
			cookie:  xcb_ewmh_get_wm_cm_owner(c.get_raw_conn(), screen as c_int),
			checked: true,
		}
	}
}

pub fn get_wm_cm_owner_unchecked(c: &Connection, screen: i32) -> GetWmCmOwnerCookie {
	unsafe {
		GetWmCmOwnerCookie {
			conn:    c,
			cookie:  xcb_ewmh_get_wm_cm_owner_unchecked(c.get_raw_conn(), screen as c_int),
			checked: false,
		}
	}
}
