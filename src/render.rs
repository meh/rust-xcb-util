use xcb;
use xcb::ffi::*;
use xcb::render::*;
use ffi::render::*;

pub const PICT_FORMAT_ID:         Pictformat = XCB_PICT_FORMAT_ID;
pub const PICT_FORMAT_TYPE:       Pictformat = XCB_PICT_FORMAT_TYPE;
pub const PICT_FORMAT_DEPTH:      Pictformat = XCB_PICT_FORMAT_DEPTH;
pub const PICT_FORMAT_RED:        Pictformat = XCB_PICT_FORMAT_RED;
pub const PICT_FORMAT_RED_MASK:   Pictformat = XCB_PICT_FORMAT_RED_MASK;
pub const PICT_FORMAT_GREEN:      Pictformat = XCB_PICT_FORMAT_GREEN;
pub const PICT_FORMAT_GREEN_MASK: Pictformat = XCB_PICT_FORMAT_GREEN_MASK;
pub const PICT_FORMAT_BLUE:       Pictformat = XCB_PICT_FORMAT_BLUE;
pub const PICT_FORMAT_BLUE_MASK:  Pictformat = XCB_PICT_FORMAT_BLUE_MASK;
pub const PICT_FORMAT_ALPHA:      Pictformat = XCB_PICT_FORMAT_ALPHA;
pub const PICT_FORMAT_ALPHA_MASK: Pictformat = XCB_PICT_FORMAT_ALPHA_MASK;
pub const PICT_FORMAT_COLORMAP:   Pictformat = XCB_PICT_FORMAT_COLORMAP;

pub type PictStandard = xcb_pict_standard_t;
pub const PICT_STANDARD_ARGB_32: PictStandard = XCB_PICT_STANDARD_ARGB_32;
pub const PICT_STANDARD_RGB_24:  PictStandard = XCB_PICT_STANDARD_RGB_24;
pub const PICT_STANDARD_A_8:     PictStandard = XCB_PICT_STANDARD_A_8;
pub const PICT_STANDARD_A_4:     PictStandard = XCB_PICT_STANDARD_A_4;
pub const PICT_STANDARD_A_1:     PictStandard = XCB_PICT_STANDARD_A_1;

pub trait QueryPictFormatsReplyExt {
    fn find_visual_format(&self, visual: xcb::Visualid) -> Option<Pictvisual>;
    fn find_format(&self, mask: Pictformat, formats: &[Pictforminfo]) -> Option<Pictforminfo>;
    fn find_standard_format(&self, format: PictStandard) -> Option<Pictforminfo>;
}

impl QueryPictFormatsReplyExt for QueryPictFormatsReply {
    fn find_visual_format(&self, visual: xcb::Visualid) -> Option<Pictvisual> {
        let result = unsafe {
            xcb_render_util_find_visual_format(
                self.ptr,
                visual,
            )
        };

        if result.is_null() {
            None
        } else {
            Some(Pictvisual {
                base: unsafe { *result },
            })
        }
    }

    fn find_format(&self, mask: Pictformat, formats: &[Pictforminfo]) -> Option<Pictforminfo> {
        let formats: Vec<xcb::ffi::render::xcb_render_pictforminfo_t>  = formats
            .into_iter()
            .map(|format| format.base)
            .collect();

        let result = unsafe {
            xcb_render_util_find_format(
                self.ptr,
                mask,
                formats.as_ptr(),
                formats.len() as _,
            )
        };

        if result.is_null() {
            None
        } else {
            Some(Pictforminfo {
                base: unsafe { *result },
            })
        }
    }

    fn find_standard_format(&self, format: PictStandard) -> Option<Pictforminfo> {
        let result = unsafe {
            xcb_render_util_find_standard_format(
                self.ptr,
                format,
            )
        };

        if result.is_null() {
            None
        } else {
            Some(Pictforminfo {
                base: unsafe { *result },
            })
        }
    }
}
