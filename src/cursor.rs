use xcb;

pub const NUM_GLYPHS: u16 = 154;
pub const X_CURSOR: u16 = 0;
pub const ARROW: u16 = 2;
pub const BASED_ARROW_DOWN: u16 = 4;
pub const BASED_ARROW_UP: u16 = 6;
pub const BOAT: u16 = 8;
pub const BOGOSITY: u16 = 10;
pub const BOTTOM_LEFT_CORNER: u16 = 12;
pub const BOTTOM_RIGHT_CORNER: u16 = 14;
pub const BOTTOM_SIDE: u16 = 16;
pub const BOTTOM_TEE: u16 = 18;
pub const BOX_SPIRAL: u16 = 20;
pub const CENTER_PTR: u16 = 22;
pub const CIRCLE: u16 = 24;
pub const CLOCK: u16 = 26;
pub const COFFEE_MUG: u16 = 28;
pub const CROSS: u16 = 30;
pub const CROSS_REVERSE: u16 = 32;
pub const CROSSHAIR: u16 = 34;
pub const DIAMOND_CROSS: u16 = 36;
pub const DOT: u16 = 38;
pub const DOTBOX: u16 = 40;
pub const DOUBLE_ARROW: u16 = 42;
pub const DRAFT_LARGE: u16 = 44;
pub const DRAFT_SMALL: u16 = 46;
pub const DRAPED_BOX: u16 = 48;
pub const EXCHANGE: u16 = 50;
pub const FLEUR: u16 = 52;
pub const GOBBLER: u16 = 54;
pub const GUMBY: u16 = 56;
pub const HAND1: u16 = 58;
pub const HAND2: u16 = 60;
pub const HEART: u16 = 62;
pub const ICON: u16 = 64;
pub const IRON_CROSS: u16 = 66;
pub const LEFT_PTR: u16 = 68;
pub const LEFT_SIDE: u16 = 70;
pub const LEFT_TEE: u16 = 72;
pub const LEFTBUTTON: u16 = 74;
pub const LL_ANGLE: u16 = 76;
pub const LR_ANGLE: u16 = 78;
pub const MAN: u16 = 80;
pub const MIDDLEBUTTON: u16 = 82;
pub const MOUSE: u16 = 84;
pub const PENCIL: u16 = 86;
pub const PIRATE: u16 = 88;
pub const PLUS: u16 = 90;
pub const QUESTION_ARROW: u16 = 92;
pub const RIGHT_PTR: u16 = 94;
pub const RIGHT_SIDE: u16 = 96;
pub const RIGHT_TEE: u16 = 98;
pub const RIGHTBUTTON: u16 = 100;
pub const RTL_LOGO: u16 = 102;
pub const SAILBOAT: u16 = 104;
pub const SB_DOWN_ARROW: u16 = 106;
pub const SB_H_DOUBLE_ARROW: u16 = 108;
pub const SB_LEFT_ARROW: u16 = 110;
pub const SB_RIGHT_ARROW: u16 = 112;
pub const SB_UP_ARROW: u16 = 114;
pub const SB_V_DOUBLE_ARROW: u16 = 116;
pub const SHUTTLE: u16 = 118;
pub const SIZING: u16 = 120;
pub const SPIDER: u16 = 122;
pub const SPRAYCAN: u16 = 124;
pub const STAR: u16 = 126;
pub const TARGET: u16 = 128;
pub const TCROSS: u16 = 130;
pub const TOP_LEFT_ARROW: u16 = 132;
pub const TOP_LEFT_CORNER: u16 = 134;
pub const TOP_RIGHT_CORNER: u16 = 136;
pub const TOP_SIDE: u16 = 138;
pub const TOP_TEE: u16 = 140;
pub const TREK: u16 = 142;
pub const UL_ANGLE: u16 = 144;
pub const UMBRELLA: u16 = 146;
pub const UR_ANGLE: u16 = 148;
pub const WATCH: u16 = 150;
pub const XTERM: u16 = 152;

pub fn create_font_cursor(c: &xcb::Connection, glyph: u16) -> xcb::Cursor {
	let font = c.generate_id();
	xcb::open_font(c, font, "cursor");

	let cursor = c.generate_id();
	xcb::create_glyph_cursor(c, cursor, font, font, glyph, glyph + 1,
		0, 0, 0, 0xffff, 0xffff, 0xffff);

	cursor
}

pub fn create_font_cursor_checked(c: &xcb::Connection, glyph: u16) -> Result<xcb::Cursor, xcb::GenericError> {
	let font = c.generate_id();
	try!(xcb::open_font_checked(c, font, "cursor").request_check());

	let cursor = c.generate_id();
	try!(xcb::create_glyph_cursor(c, cursor, font, font, glyph, glyph + 1,
		0, 0, 0, 0xffff, 0xffff, 0xffff).request_check());

	Ok(cursor)
}
