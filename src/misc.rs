use xcb;
use icccm;

pub fn client_window(c: &xcb::Connection, window: xcb::Window) -> Option<xcb::Window> {
	fn try_children(c: &xcb::Connection, window: xcb::Window) -> Option<xcb::Window> {
		if let Ok(query) = xcb::query_tree(c, window).get_reply() {
			for &child in query.children() {
				if icccm::get_wm_state(c, child).get_reply().is_ok() {
					return Some(child);
				}

				if let Some(window) = try_children(c, child) {
					return Some(window);
				}
			}
		}

		None
	}

	if icccm::get_wm_state(c, window).get_reply().is_ok() {
		Some(window)
	}
	else {
		try_children(c, window)
	}
}
