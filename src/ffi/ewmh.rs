use std::mem;
use std::ptr;

use xcb::ffi::*;
use libc::{c_char, c_int, c_uint};
use libc::{free};

#[repr(C)]
pub struct xcb_ewmh_connection_t {
	pub connection: *mut xcb_connection_t,
	pub screens:    *mut *mut xcb_screen_t,
	pub nb_screens: c_int,

	pub _NET_WM_CM_Sn: *mut xcb_atom_t,
	pub _NET_SUPPORTED: xcb_atom_t,
	pub _NET_CLIENT_LIST: xcb_atom_t,
	pub _NET_CLIENT_LIST_STACKING: xcb_atom_t,
	pub _NET_NUMBER_OF_DESKTOPS: xcb_atom_t,
	pub _NET_DESKTOP_GEOMETRY: xcb_atom_t,
	pub _NET_DESKTOP_VIEWPORT: xcb_atom_t,
	pub _NET_CURRENT_DESKTOP: xcb_atom_t,
	pub _NET_DESKTOP_NAMES: xcb_atom_t,
	pub _NET_ACTIVE_WINDOW: xcb_atom_t,
	pub _NET_WORKAREA: xcb_atom_t,
	pub _NET_SUPPORTING_WM_CHECK: xcb_atom_t,
	pub _NET_VIRTUAL_ROOTS: xcb_atom_t,
	pub _NET_DESKTOP_LAYOUT: xcb_atom_t,
	pub _NET_SHOWING_DESKTOP: xcb_atom_t,
	pub _NET_CLOSE_WINDOW: xcb_atom_t,
	pub _NET_MOVERESIZE_WINDOW: xcb_atom_t,
	pub _NET_WM_MOVERESIZE: xcb_atom_t,
	pub _NET_RESTACK_WINDOW: xcb_atom_t,
	pub _NET_REQUEST_FRAME_EXTENTS: xcb_atom_t,
	pub _NET_WM_NAME: xcb_atom_t,
	pub _NET_WM_VISIBLE_NAME: xcb_atom_t,
	pub _NET_WM_ICON_NAME: xcb_atom_t,
	pub _NET_WM_VISIBLE_ICON_NAME: xcb_atom_t,
	pub _NET_WM_DESKTOP: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE: xcb_atom_t,
	pub _NET_WM_STATE: xcb_atom_t,
	pub _NET_WM_ALLOWED_ACTIONS: xcb_atom_t,
	pub _NET_WM_STRUT: xcb_atom_t,
	pub _NET_WM_STRUT_PARTIAL: xcb_atom_t,
	pub _NET_WM_ICON_GEOMETRY: xcb_atom_t,
	pub _NET_WM_ICON: xcb_atom_t,
	pub _NET_WM_PID: xcb_atom_t,
	pub _NET_WM_HANDLED_ICONS: xcb_atom_t,
	pub _NET_WM_USER_TIME: xcb_atom_t,
	pub _NET_WM_USER_TIME_WINDOW: xcb_atom_t,
	pub _NET_FRAME_EXTENTS: xcb_atom_t,
	pub _NET_WM_PING: xcb_atom_t,
	pub _NET_WM_SYNC_REQUEST: xcb_atom_t,
	pub _NET_WM_SYNC_REQUEST_COUNTER: xcb_atom_t,
	pub _NET_WM_FULLSCREEN_MONITORS: xcb_atom_t,
	pub _NET_WM_FULL_PLACEMENT: xcb_atom_t,
	pub UTF8_STRING: xcb_atom_t,
	pub WM_PROTOCOLS: xcb_atom_t,
	pub MANAGER: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_DESKTOP: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_DOCK: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_TOOLBAR: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_MENU: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_UTILITY: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_SPLASH: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_DIALOG: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_DROPDOWN_MENU: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_POPUP_MENU: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_TOOLTIP: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_NOTIFICATION: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_COMBO: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_DND: xcb_atom_t,
	pub _NET_WM_WINDOW_TYPE_NORMAL: xcb_atom_t,
	pub _NET_WM_STATE_MODAL: xcb_atom_t,
	pub _NET_WM_STATE_STICKY: xcb_atom_t,
	pub _NET_WM_STATE_MAXIMIZED_VERT: xcb_atom_t,
	pub _NET_WM_STATE_MAXIMIZED_HORZ: xcb_atom_t,
	pub _NET_WM_STATE_SHADED: xcb_atom_t,
	pub _NET_WM_STATE_SKIP_TASKBAR: xcb_atom_t,
	pub _NET_WM_STATE_SKIP_PAGER: xcb_atom_t,
	pub _NET_WM_STATE_HIDDEN: xcb_atom_t,
	pub _NET_WM_STATE_FULLSCREEN: xcb_atom_t,
	pub _NET_WM_STATE_ABOVE: xcb_atom_t,
	pub _NET_WM_STATE_BELOW: xcb_atom_t,
	pub _NET_WM_STATE_DEMANDS_ATTENTION: xcb_atom_t,
	pub _NET_WM_ACTION_MOVE: xcb_atom_t,
	pub _NET_WM_ACTION_RESIZE: xcb_atom_t,
	pub _NET_WM_ACTION_MINIMIZE: xcb_atom_t,
	pub _NET_WM_ACTION_SHADE: xcb_atom_t,
	pub _NET_WM_ACTION_STICK: xcb_atom_t,
	pub _NET_WM_ACTION_MAXIMIZE_HORZ: xcb_atom_t,
	pub _NET_WM_ACTION_MAXIMIZE_VERT: xcb_atom_t,
	pub _NET_WM_ACTION_FULLSCREEN: xcb_atom_t,
	pub _NET_WM_ACTION_CHANGE_DESKTOP: xcb_atom_t,
	pub _NET_WM_ACTION_CLOSE: xcb_atom_t,
	pub _NET_WM_ACTION_ABOVE: xcb_atom_t,
	pub _NET_WM_ACTION_BELOW: xcb_atom_t,
}

#[repr(C)]
pub struct xcb_ewmh_get_atoms_reply_t {
	pub atoms_len: u32,
	pub atoms: *mut xcb_atom_t,

	_reply: *mut xcb_get_property_reply_t,
}

#[repr(C)]
pub struct xcb_ewmh_get_windows_reply_t {
	pub windows_len: u32,
	pub windows: *mut xcb_window_t,

	_reply: *mut xcb_get_property_reply_t,
}

#[repr(C)]
pub struct xcb_ewmh_get_utf8_strings_reply_t {
	pub strings_len: u32,
	pub strings: *mut c_char,

	_reply: *mut xcb_get_property_reply_t,
}

#[repr(C)]
pub struct xcb_ewmh_coordinates_t {
	pub x: u32,
	pub y: u32,
}

#[repr(C)]
pub struct xcb_ewmh_get_desktop_viewport_reply_t {
	pub desktop_viewport_len: u32,
	pub desktop_viewport: *mut xcb_ewmh_coordinates_t,

	_reply: *mut xcb_get_property_reply_t,
}

#[repr(C)]
pub struct xcb_ewmh_geometry_t {
	pub x: u32,
	pub y: u32,
	pub width: u32,
	pub height: u32,
}

#[repr(C)]
pub struct xcb_ewmh_get_workarea_reply_t {
	pub workarea_len: u32,
	pub workarea: *mut xcb_ewmh_geometry_t,

	_reply: *mut xcb_get_property_reply_t,
}

pub type xcb_ewmh_client_source_type_t = u32;
pub const XCB_EWMH_CLIENT_SOURCE_TYPE_NONE:   xcb_ewmh_client_source_type_t = 0;
pub const XCB_EWMH_CLIENT_SOURCE_TYPE_NORMAL: xcb_ewmh_client_source_type_t = 1;
pub const XCB_EWMH_CLIENT_SOURCE_TYPE_OTHER:  xcb_ewmh_client_source_type_t = 2;

pub type xcb_ewmh_desktop_layout_orientation_t = u32;
pub const XCB_EWMH_WM_ORIENTATION_HORZ: xcb_ewmh_desktop_layout_orientation_t = 0;
pub const XCB_EWMH_WM_ORIENTATION_VERT: xcb_ewmh_desktop_layout_orientation_t = 1;

pub type xcb_ewmh_desktop_layout_starting_corner_t = u32;
pub const XCB_EWMH_WM_TOPLEFT:     xcb_ewmh_desktop_layout_starting_corner_t = 0;
pub const XCB_EWMH_WM_TOPRIGHT:    xcb_ewmh_desktop_layout_starting_corner_t = 1;
pub const XCB_EWMH_WM_BOTTOMRIGHT: xcb_ewmh_desktop_layout_starting_corner_t = 2;
pub const XCB_EWMH_WM_BOTTOMLEFT:  xcb_ewmh_desktop_layout_starting_corner_t = 3;

#[repr(C)]
pub struct xcb_ewmh_get_desktop_layout_reply_t {
	pub orientation: u32,
	pub columns: u32,
	pub rows: u32,
	pub starting_corner: u32,
}

pub type xcb_ewmh_moveresize_window_opt_flags_t = u32;
pub const XCB_EWMH_MOVERESIZE_WINDOW_X:      xcb_ewmh_moveresize_window_opt_flags_t = 1 << 8;
pub const XCB_EWMH_MOVERESIZE_WINDOW_Y:      xcb_ewmh_moveresize_window_opt_flags_t = 1 << 9;
pub const XCB_EWMH_MOVERESIZE_WINDOW_WIDTH:  xcb_ewmh_moveresize_window_opt_flags_t = 1 << 10;
pub const XCB_EWMH_MOVERESIZE_WINDOW_HEIGHT: xcb_ewmh_moveresize_window_opt_flags_t = 1 << 11;

pub type xcb_ewmh_moveresize_direction_t = u32;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_TOPLEFT:     xcb_ewmh_moveresize_direction_t = 0;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_TOP:         xcb_ewmh_moveresize_direction_t = 1;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_TOPRIGHT:    xcb_ewmh_moveresize_direction_t = 2;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_RIGHT:       xcb_ewmh_moveresize_direction_t = 3;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOMRIGHT: xcb_ewmh_moveresize_direction_t = 4;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOM:      xcb_ewmh_moveresize_direction_t = 5;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_BOTTOMLEFT:  xcb_ewmh_moveresize_direction_t = 6;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_LEFT:        xcb_ewmh_moveresize_direction_t = 7;
pub const XCB_EWMH_WM_MOVERESIZE_MOVE:             xcb_ewmh_moveresize_direction_t = 8;
pub const XCB_EWMH_WM_MOVERESIZE_SIZE_KEYBOARD:    xcb_ewmh_moveresize_direction_t = 9;
pub const XCB_EWMH_WM_MOVERESIZE_MOVE_KEYBOARD:    xcb_ewmh_moveresize_direction_t = 10;
pub const XCB_EWMH_WM_MOVERESIZE_CANCEL:           xcb_ewmh_moveresize_direction_t = 11;

pub type xcb_ewmh_wm_state_action_t = u32;
pub const XCB_EWMH_WM_STATE_REMOVE: xcb_ewmh_wm_state_action_t = 0;
pub const XCB_EWMH_WM_STATE_ADD:    xcb_ewmh_wm_state_action_t = 1;
pub const XCB_EWMH_WM_STATE_TOGGLE: xcb_ewmh_wm_state_action_t = 2;

#[repr(C)]
pub struct xcb_ewmh_wm_strut_partial_t {
	pub left: u32,
	pub right: u32,
	pub top: u32,
	pub bottom: u32,
	pub left_start_y: u32,
	pub left_end_y: u32,
	pub right_start_y: u32,
	pub right_end_y: u32,
	pub top_start_x: u32,
	pub top_end_x: u32,
	pub bottom_start_x: u32,
	pub bottom_end_x: u32,
}

#[repr(C)]
pub struct xcb_ewmh_wm_icon_iterator_t {
	pub width: u32,
	pub height: u32,
	pub data: *mut u32,
	pub rem: c_uint,
	pub index: c_uint,
}

#[repr(C)]
pub struct xcb_ewmh_get_wm_icon_reply_t {
	pub num_icons: c_uint,

	_reply: *mut xcb_get_property_reply_t,
}

#[repr(C)]
pub struct xcb_ewmh_get_extents_reply_t {
	pub top: u32,
	pub bottom: u32,
	pub left: u32,
	pub right: u32,
}

#[repr(C)]
pub struct xcb_ewmh_get_wm_fullscreen_monitors_reply_t {
	pub top: u32,
	pub bottom: u32,
	pub left: u32,
	pub right: u32,
}

#[cfg_attr(feature = "static", link(name = "xcb-ewmh", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "xcb-ewmh"))]
extern "C" {
	pub fn xcb_ewmh_init_atoms(c: *mut xcb_connection_t, ewmh: *mut xcb_ewmh_connection_t) -> *mut xcb_intern_atom_cookie_t;
	pub fn xcb_ewmh_init_atoms_replies(ewmh: *mut xcb_ewmh_connection_t, ewmh_cookies: *mut xcb_intern_atom_cookie_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_send_client_message(c: *mut xcb_connection_t, window: xcb_window_t, dest: xcb_window_t, atom: xcb_atom_t, data_len: u32, data: *const u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_close_window(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, window_to_close: xcb_window_t, timetamp: xcb_timestamp_t, source_indication: xcb_ewmh_client_source_type_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_moveresize_window(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, moveresize_window: xcb_window_t, gravity: xcb_gravity_t, source_indication: xcb_ewmh_client_source_type_t, flags: xcb_ewmh_moveresize_window_opt_flags_t, x: u32, y: u32, width: u32, height: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_wm_moveresize(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, moveresize_window: xcb_window_t, x_root: u32, y_root: u32, direction: xcb_ewmh_moveresize_direction_t, button: xcb_button_index_t, source_indication: xcb_ewmh_client_source_type_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_restack_window(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, window_to_restack: xcb_window_t, sibling_window: xcb_window_t, detail: xcb_stack_mode_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_send_wm_ping(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, timestamp: xcb_timestamp_t) -> xcb_void_cookie_t;

	pub fn xcb_ewmh_get_window_from_reply(window: *mut xcb_window_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_window_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, window: *mut xcb_window_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_get_cardinal_from_reply(cardinal: *mut u32, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_cardinal_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, cardinal: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_get_atoms_from_reply(atoms: *mut xcb_ewmh_get_atoms_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_atoms_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, atoms: *mut xcb_ewmh_get_atoms_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_ewmh_get_atoms_reply_wipe(data: *mut xcb_ewmh_get_atoms_reply_t);

	pub fn xcb_ewmh_get_windows_from_reply(atoms: *mut xcb_ewmh_get_windows_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_windows_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, atoms: *mut xcb_ewmh_get_windows_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_ewmh_get_windows_reply_wipe(data: *mut xcb_ewmh_get_windows_reply_t);

	pub fn xcb_ewmh_get_utf8_strings_from_reply(ewmh: *mut xcb_ewmh_connection_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_utf8_strings_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_ewmh_get_utf8_strings_reply_wipe(data: *mut xcb_ewmh_get_utf8_strings_reply_t);

	pub fn xcb_ewmh_set_supported(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_supported_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_supported(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_supported_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_client_list(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_client_list_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_client_list(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_client_list_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_client_list_stacking(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_client_list_stacking_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_client_list_stacking(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_client_list_stacking_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_number_of_desktops(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, number_of_desktops: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_number_of_desktops_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, number_of_desktops: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_number_of_desktops(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_number_of_desktops_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_desktop_geometry(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_width: u32, new_height: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_desktop_geometry_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_width: u32, new_height: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_desktop_geometry(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_desktop_geometry_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_request_change_desktop_geometry(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_width: u32, new_height: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_desktop_geometry_from_reply(width: *mut u32, height: *mut u32, r: xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_desktop_geometry_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, width: *mut u32, height: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_desktop_viewport(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_ewmh_coordinates_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_desktop_viewport_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_ewmh_coordinates_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_desktop_viewport(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_desktop_viewport_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_desktop_viewport_from_reply(vp: *mut xcb_ewmh_get_desktop_viewport_reply_t, r: xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_desktop_viewport_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, vp: *mut xcb_ewmh_get_desktop_viewport_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_ewmh_get_desktop_viewport_reply_wipe(r: *mut xcb_ewmh_get_desktop_viewport_reply_t);

	pub fn xcb_ewmh_set_current_desktop(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_current_desktop: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_current_desktop_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_current_desktop: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_current_desktop(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_current_desktop_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_request_change_current_desktop(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_desktop: u32, timestamp: xcb_timestamp_t) -> xcb_void_cookie_t;

	pub fn xcb_ewmh_set_desktop_names(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, strings_len: u32, strings: *mut c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_desktop_names_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, strings_len: u32, strings: *mut c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_desktop_names(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_desktop_names_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_active_window(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_active_window: xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_active_window_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, new_active_window: xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_change_active_window(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, window_to_activate: xcb_window_t, source_indication: xcb_ewmh_client_source_type_t, timestamp: xcb_timestamp_t, current_active_window: xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_active_window(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_active_window_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_workarea(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_ewmh_geometry_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_workarea_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_ewmh_geometry_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_workarea(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_workarea_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_workarea_from_reply(wa: *mut xcb_ewmh_get_workarea_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_workarea_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, wa: *mut xcb_ewmh_get_workarea_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_ewmh_get_workarea_reply_wipe(r: *mut xcb_ewmh_get_workarea_reply_t);

	pub fn xcb_ewmh_set_supporting_wm_check(ewmh: *mut xcb_ewmh_connection_t, parent_window: xcb_window_t, child_window: xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_supporting_wm_check_checked(ewmh: *mut xcb_ewmh_connection_t, parent_window: xcb_window_t, child_window: xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_supporting_wm_check(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_supporting_wm_check_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_virtual_roots(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_virtual_roots_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, list_len: u32, list: *const xcb_window_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_virtual_roots(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_virtual_roots_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_desktop_layout(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, orientation: xcb_ewmh_desktop_layout_orientation_t, columns: u32, rows: u32, starting_corner: xcb_ewmh_desktop_layout_starting_corner_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_desktop_layout_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, orientation: xcb_ewmh_desktop_layout_orientation_t, columns: u32, rows: u32, starting_corner: xcb_ewmh_desktop_layout_starting_corner_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_desktop_layout(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_desktop_layout_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_desktop_layout_from_reply(desktop_layouts: *mut xcb_ewmh_get_desktop_layout_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_desktop_layout_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, desktop_layouts: *mut xcb_ewmh_get_desktop_layout_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_showing_desktop(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, desktop: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_showing_desktop_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, desktop: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_showing_desktop(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_showing_desktop_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_name_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_name_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_visible_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_visible_name_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_visible_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_visible_name_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_icon_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_icon_name_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_icon_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_icon_name_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_visible_icon_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_visible_icon_name_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, strings_len: u32, strings: *const c_char) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_visible_icon_name(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_visible_icon_name_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_desktop(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, desktop: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_desktop_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, desktop: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_change_wm_desktop(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, client_window: xcb_window_t, new_desktop: u32, source_indication: xcb_ewmh_client_source_type_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_desktop(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_desktop_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_window_type(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_window_type_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_window_type(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_window_type_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_window_type_from_reply(wtypes: *mut xcb_ewmh_get_atoms_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_window_type_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, name: *mut xcb_ewmh_get_atoms_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_state(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_state_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_change_wm_state(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, client_window: xcb_window_t, action: xcb_ewmh_wm_state_action_t, first_property: xcb_atom_t, second_property: xcb_atom_t, source_indication: xcb_ewmh_client_source_type_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_state(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_state_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_state_from_reply(wtypes: *mut xcb_ewmh_get_atoms_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_state_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, name: *mut xcb_ewmh_get_atoms_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_allowed_actions(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_allowed_actions_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, list_len: u32, list: *const xcb_atom_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_allowed_actions(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_allowed_actions_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_allowed_actions_from_reply(wtypes: *mut xcb_ewmh_get_atoms_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_allowed_actions_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, name: *mut xcb_ewmh_get_atoms_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_strut(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, left: u32, right: u32, top: u32, bottom: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_strut_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, left: u32, right: u32, top: u32, bottom: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_strut(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_strut_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_strut_from_reply(struts: *mut xcb_ewmh_get_extents_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_strut_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, struts: *mut xcb_ewmh_get_extents_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_strut_partial(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, wm_strut: xcb_ewmh_wm_strut_partial_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_strut_partial_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, wm_strut: xcb_ewmh_wm_strut_partial_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_strut_partial(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_strut_partial_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_strut_partial_from_reply(struts: *mut xcb_ewmh_wm_strut_partial_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_strut_partial_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, struts: *mut xcb_ewmh_wm_strut_partial_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_icon_geometry(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, left: u32, right: u32, top: u32, bottom: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_icon_geometry_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, left: u32, right: u32, top: u32, bottom: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_icon_geometry(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_icon_geometry_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_icon_geometry_from_reply(icons: *mut xcb_ewmh_geometry_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_icon_geometry_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, icons: *mut xcb_ewmh_geometry_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_append_wm_icon(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, width: u32, height: u32, img_len: u32, img: *const u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_append_wm_icon_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, width: u32, height: u32, img_len: u32, img: *const u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_icon(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_icon_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_icon_from_reply(wm_icon: *mut xcb_ewmh_get_wm_icon_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_icon_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, wm_icon: *mut xcb_ewmh_get_wm_icon_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;
	pub fn xcb_ewmh_get_wm_icon_iterator(wm_icon: *const xcb_ewmh_get_wm_icon_reply_t) -> xcb_ewmh_wm_icon_iterator_t;
	pub fn xcb_ewmh_get_wm_icon_length(wm_icon: *const xcb_ewmh_get_wm_icon_reply_t) -> c_uint;
	pub fn xcb_ewmh_get_wm_icon_next(iterator: *mut xcb_ewmh_wm_icon_iterator_t);
	pub fn xcb_ewmh_get_wm_icon_reply_wipe(wm_icon: *mut xcb_ewmh_get_wm_icon_reply_t);

	pub fn xcb_ewmh_set_wm_pid(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, pid: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_pid_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, pid: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_pid(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_pid_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_handled_icons(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, handled_icons: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_handled_icons_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, handled_icons: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_handled_icons(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_handled_icons_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_user_time(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, time: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_user_time_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, time: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_user_time(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_user_time_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_wm_user_time_window(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, time: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_user_time_window_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, time: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_user_time_window(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_user_time_window_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;

	pub fn xcb_ewmh_set_frame_extents(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, left: u32, right: u32, top: u32, bottom: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_frame_extents_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, left: u32, right: u32, top: u32, bottom: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_frame_extents(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_frame_extents_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_frame_extents_from_reply(frame_extents: *mut xcb_ewmh_get_extents_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_frame_extents_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, frame_extents: *mut xcb_ewmh_get_extents_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_sync_request_counter(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, atom: xcb_atom_t, low: u32, high: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_sync_request_counter_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, atom: xcb_atom_t, low: u32, high: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_send_wm_sync_rqeuest(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, wm_protocols: xcb_atom_t, wm_sync_request: xcb_atom_t, timestamp: xcb_timestamp_t, couner: u64) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_sync_request_counter(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_sync_request_counter_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_sync_request_counter_from_reply(counter: *mut u64) -> u8;
	pub fn xcb_ewmh_get_wm_sync_request_counter_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, counter: *mut u64, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_fullscreen_monitors(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, top: u32, bottom: u32, left: u32, right: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_fullscreen_monitors_checked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t, top: u32, bottom: u32, left: u32, right: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_request_change_wm_fullscreen_monitors(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, window: xcb_window_t, top: u32, bottom: u32, left: u32, right: u32, source_indication: xcb_ewmh_client_source_type_t) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_fullscreen_monitors(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_fullscreen_monitors_unchecked(ewmh: *mut xcb_ewmh_connection_t, window: xcb_window_t) -> xcb_get_property_cookie_t;
	pub fn xcb_ewmh_get_wm_fullscreen_monitors_from_reply(monitors: *mut xcb_ewmh_get_wm_fullscreen_monitors_reply_t, r: *const xcb_get_property_reply_t) -> u8;
	pub fn xcb_ewmh_get_wm_fullscreen_monitors_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, monitors: *mut xcb_ewmh_get_wm_fullscreen_monitors_reply_t, e: *mut *mut xcb_generic_error_t) -> u8;

	pub fn xcb_ewmh_set_wm_cm_owner(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, owner: xcb_window_t, timestamp: xcb_timestamp_t, selection_data1: u32, selection_data2: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_set_wm_cm_owner_checked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, owner: xcb_window_t, timestamp: xcb_timestamp_t, selection_data1: u32, selection_data2: u32) -> xcb_void_cookie_t;
	pub fn xcb_ewmh_get_wm_cm_owner(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_selection_owner_cookie_t;
	pub fn xcb_ewmh_get_wm_cm_owner_unchecked(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int) -> xcb_get_selection_owner_cookie_t;
	pub fn xcb_ewmh_get_wm_cm_owner_from_reply(owner: *mut xcb_window_t, r: *const xcb_get_selection_owner_cookie_t) -> u8;
	pub fn xcb_ewmh_get_wm_cm_owner_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_selection_owner_cookie_t, owner: *mut xcb_window_t, e: *mut *mut xcb_generic_error_t) -> u8;
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_connection_wipe(ewmh: *mut xcb_ewmh_connection_t) {
	free((*ewmh).screens as *mut _);
	free((*ewmh)._NET_WM_CM_Sn as *mut _);
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_supported_from_reply(supported: *mut xcb_ewmh_get_atoms_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_atoms_from_reply(supported, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_supported_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, supported: *mut xcb_ewmh_get_atoms_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_atoms_reply(ewmh, cookie, supported, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_client_list_from_reply(clients: *mut xcb_ewmh_get_windows_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_windows_from_reply(clients, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_client_list_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, clients: *mut xcb_ewmh_get_windows_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_windows_reply(ewmh, cookie, clients, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_client_list_stacking_from_reply(clients: *mut xcb_ewmh_get_windows_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_windows_from_reply(clients, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_client_list_stacking_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, clients: *mut xcb_ewmh_get_windows_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_windows_reply(ewmh, cookie, clients, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_number_of_desktops_from_reply(number_of_desktops: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(number_of_desktops, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_number_of_desktops_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, number_of_desktops: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, number_of_desktops, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_current_desktop_from_reply(current_desktop: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(current_desktop, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_current_desktop_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, current_desktop: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, current_desktop, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_desktop_names_from_reply(ewmh: *mut xcb_ewmh_connection_t, names: *mut xcb_ewmh_get_utf8_strings_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_utf8_strings_from_reply(ewmh, names, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_desktop_names_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, names: *mut xcb_ewmh_get_utf8_strings_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_utf8_strings_reply(ewmh, cookie, names, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_active_window_from_reply(active_window: *mut xcb_window_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_window_from_reply(active_window, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_active_window_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, active_window: *mut xcb_window_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_window_reply(ewmh, cookie, active_window, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_supporting_wm_check_from_reply(window: *mut xcb_window_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_window_from_reply(window, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_supporting_wm_check_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, window: *mut xcb_window_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_window_reply(ewmh, cookie, window, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_virtual_roots_from_reply(virtual_roots: *mut xcb_ewmh_get_windows_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_windows_from_reply(virtual_roots, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_virtual_roots_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, virtual_roots: *mut xcb_ewmh_get_windows_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_windows_reply(ewmh, cookie, virtual_roots, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_showing_desktop_from_reply(desktop: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(desktop, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_showing_desktop_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, desktop: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, desktop, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_request_change_showing_desktop(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, enter: u32) -> xcb_void_cookie_t {
	xcb_ewmh_send_client_message((*ewmh).connection, XCB_NONE, (**(*ewmh).screens.offset(screen_nbr as isize)).root,
		(*ewmh)._NET_SHOWING_DESKTOP, mem::size_of_val(&enter) as u32, &enter)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_request_frame_extents(ewmh: *mut xcb_ewmh_connection_t, screen_nbr: c_int, client_window: xcb_window_t) -> xcb_void_cookie_t {
	xcb_ewmh_send_client_message((*ewmh).connection, client_window, (**(*ewmh).screens.offset(screen_nbr as isize)).root,
		(*ewmh)._NET_REQUEST_FRAME_EXTENTS, 0, ptr::null())
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_name_from_reply(ewmh: *mut xcb_ewmh_connection_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_utf8_strings_from_reply(ewmh, data, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_name_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_utf8_strings_reply(ewmh, cookie, data, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_visible_name_from_reply(ewmh: *mut xcb_ewmh_connection_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_utf8_strings_from_reply(ewmh, data, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_visible_name_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_utf8_strings_reply(ewmh, cookie, data, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_icon_name_from_reply(ewmh: *mut xcb_ewmh_connection_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_utf8_strings_from_reply(ewmh, data, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_icon_name_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_utf8_strings_reply(ewmh, cookie, data, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_visible_icon_name_from_reply(ewmh: *mut xcb_ewmh_connection_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_utf8_strings_from_reply(ewmh, data, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_visible_icon_name_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, data: *mut xcb_ewmh_get_utf8_strings_reply_t, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_utf8_strings_reply(ewmh, cookie, data, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_desktop_from_reply(desktop: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(desktop, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_desktop_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, desktop: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, desktop, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_set_wm_icon(ewmh: *mut xcb_ewmh_connection_t, mode: u8, window: xcb_window_t, data_len: u32, data: *const u32) -> xcb_void_cookie_t {
	xcb_change_property((*ewmh).connection, mode, window, (*ewmh)._NET_WM_ICON, XCB_ATOM_CARDINAL, 32, data_len, data as *const _)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_set_wm_icon_checked(ewmh: *mut xcb_ewmh_connection_t, mode: u8, window: xcb_window_t, data_len: u32, data: *const u32) -> xcb_void_cookie_t {
	xcb_change_property_checked((*ewmh).connection, mode, window, (*ewmh)._NET_WM_ICON, XCB_ATOM_CARDINAL, 32, data_len, data as *const _)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_pid_from_reply(pid: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(pid, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_pid_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, pid: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, pid, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_handled_icons_from_reply(handled_icons: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(handled_icons, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_handled_icons_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, handled_icons: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, handled_icons, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_user_time_from_reply(time: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(time, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_user_time_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, time: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, time, e)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_user_time_window_from_reply(time: *mut u32, r: *const xcb_get_property_reply_t) -> u8 {
	xcb_ewmh_get_cardinal_from_reply(time, r)
}

#[inline(always)]
pub unsafe extern "C" fn xcb_ewmh_get_wm_user_time_window_reply(ewmh: *mut xcb_ewmh_connection_t, cookie: xcb_get_property_cookie_t, time: *mut u32, e: *mut *mut xcb_generic_error_t) -> u8 {
	xcb_ewmh_get_cardinal_reply(ewmh, cookie, time, e)
}
