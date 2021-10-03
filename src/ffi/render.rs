use xcb::ffi::*;
use xcb::ffi::render::*;

pub type xcb_pict_format_t = u32;
pub const XCB_PICT_FORMAT_ID:         xcb_pict_format_t = 1 << 0;
pub const XCB_PICT_FORMAT_TYPE:       xcb_pict_format_t = 1 << 1;
pub const XCB_PICT_FORMAT_DEPTH:      xcb_pict_format_t = 1 << 2;
pub const XCB_PICT_FORMAT_RED:        xcb_pict_format_t = 1 << 3;
pub const XCB_PICT_FORMAT_RED_MASK:   xcb_pict_format_t = 1 << 4;
pub const XCB_PICT_FORMAT_GREEN:      xcb_pict_format_t = 1 << 5;
pub const XCB_PICT_FORMAT_GREEN_MASK: xcb_pict_format_t = 1 << 6;
pub const XCB_PICT_FORMAT_BLUE:       xcb_pict_format_t = 1 << 7;
pub const XCB_PICT_FORMAT_BLUE_MASK:  xcb_pict_format_t = 1 << 8;
pub const XCB_PICT_FORMAT_ALPHA:      xcb_pict_format_t = 1 << 9;
pub const XCB_PICT_FORMAT_ALPHA_MASK: xcb_pict_format_t = 1 << 10;
pub const XCB_PICT_FORMAT_COLORMAP:   xcb_pict_format_t = 1 << 11;

pub type xcb_pict_standard_t = u32;
pub const XCB_PICT_STANDARD_ARGB_32: xcb_pict_standard_t = 0;
pub const XCB_PICT_STANDARD_RGB_24:  xcb_pict_standard_t = 1;
pub const XCB_PICT_STANDARD_A_8:     xcb_pict_standard_t = 2;
pub const XCB_PICT_STANDARD_A_4:     xcb_pict_standard_t = 3;
pub const XCB_PICT_STANDARD_A_1:     xcb_pict_standard_t = 4;

#[repr(C)]
pub struct xcb_render_util_composite_text_stream_t([u8; 0]);

#[cfg_attr(feature = "static", link(name = "xcb-render-util", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "xcb-render-util"))]
extern "C" {
    pub fn xcb_render_util_find_visual_format(
        formats: *const xcb_render_query_pict_formats_reply_t,
        visual: xcb_visualid_t,
    ) -> *mut xcb_render_pictvisual_t;

    pub fn xcb_render_util_find_format(
        formats: *const xcb_render_query_pict_formats_reply_t,
        mask: u32,
        ptemplate: *const xcb_render_pictforminfo_t,
        count: i32,
    ) -> *mut xcb_render_pictforminfo_t;

    pub fn xcb_render_util_find_standard_format(
        formats: *const xcb_render_query_pict_formats_reply_t,
        format: xcb_pict_standard_t,
    ) -> *mut xcb_render_pictforminfo_t;

    pub fn xcb_render_util_query_version(
        c: *mut xcb_connection_t,
    ) -> *const xcb_render_query_version_reply_t;
    pub fn xcb_render_util_query_formats(
        c: *mut xcb_connection_t,
    ) -> *const xcb_render_query_pict_formats_reply_t;
    pub fn xcb_render_util_disconnect(
        c: *mut xcb_connection_t,
    ) -> i32;

    pub fn xcb_render_util_composite_text_stream(
        initial_glyphset: xcb_render_glyphset_t,
        total_glyphs: u32,
        total_glyphset_changes: u32,
    ) -> *mut xcb_render_util_composite_text_stream_t;

    pub fn xcb_render_util_glyphs_8(
        stream: *mut xcb_render_util_composite_text_stream_t,
        dx: i16,
        dy: i16,
        count: u32,
        glyphs: *const u8,
    );
    pub fn xcb_render_util_glyphs_16(
        stream: *mut xcb_render_util_composite_text_stream_t,
        dx: i16,
        dy: i16,
        count: u32,
        glyphs: *const u8,
    );
    pub fn xcb_render_util_glyphs_32(
        stream: *mut xcb_render_util_composite_text_stream_t,
        dx: i16,
        dy: i16,
        count: u32,
        glyphs: *const u8,
    );

    pub fn xcb_render_util_change_glyphset(
        stream: *mut xcb_render_util_composite_text_stream_t,
        glyphset: xcb_render_glyphset_t,
    );

    pub fn xcb_render_util_composite_text(
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_picture_t,
        src_x: i16,
        src_y: i16,
        stream: *mut xcb_render_util_composite_text_stream_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_render_util_composite_text_checked(
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_picture_t,
        src_x: i16,
        src_y: i16,
        stream: *mut xcb_render_util_composite_text_stream_t,
    ) -> xcb_void_cookie_t;

    pub fn xcb_render_util_composite_text_free(
        stream: *mut xcb_render_util_composite_text_stream_t,
    );
}
