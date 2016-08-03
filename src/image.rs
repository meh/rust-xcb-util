use xcb;
use ffi::image::*;

pub struct Image(*mut xcb_image_t);

impl Image {
	pub fn width(&self) -> u16 {
		unsafe {
			(*self.0).width
		}
	}

	pub fn height(&self) -> u16 {
		unsafe {
			(*self.0).height
		}
	}

	pub fn put(&mut self, x: u32, y: u32, pixel: u32) {
		unsafe {
			xcb_image_put_pixel(self.0, x, y, pixel)
		}
	}

	pub fn get(&self, x: u32, y: u32) -> u32 {
		unsafe {
			xcb_image_get_pixel(self.0, x, y)
		}
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		unsafe {
			xcb_image_destroy(self.0);
		}
	}
}

pub fn annotate(image: &mut Image) {
	unsafe {
		xcb_image_annotate(image.0)
	}
}

pub fn get(c: &xcb::Connection, drawable: xcb::Drawable, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: xcb::ImageFormat) -> Result<Image, ()> {
	unsafe {
		let ptr = xcb_image_get(c.get_raw_conn(), drawable, x, y, width, height, plane_mask, format);

		if ptr.is_null() {
			Err(())
		}
		else {
			Ok(Image(ptr))
		}
	}
}

pub fn put(c: &xcb::Connection, drawable: xcb::Drawable, gc: xcb::Gcontext, image: &Image, x: i16, y: i16) -> xcb::VoidCookie {
	unsafe {
		xcb::VoidCookie {
			cookie:  xcb_image_put(c.get_raw_conn(), drawable, gc, image.0, x, y, 0),
			conn:    c.get_raw_conn(),
			checked: false,
		}
	}
}

pub fn is_native(c: &xcb::Connection, image: &Image) -> bool {
	unsafe {
		xcb_image_native(c.get_raw_conn(), image.0, 0) == image.0
	}
}

pub fn to_native(c: &xcb::Connection, image: &Image) -> Option<Image> {
	unsafe {
		let ptr = xcb_image_native(c.get_raw_conn(), image.0, 1);

		if ptr == image.0 || ptr.is_null() {
			None
		}
		else {
			Some(Image(ptr))
		}
	}
}
