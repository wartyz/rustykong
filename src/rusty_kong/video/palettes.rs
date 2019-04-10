use super::common::*;

use sdl2::pixels::Color;
use std::iter::Iterator;

pub fn get_palette_colors() ->Vec<Color>{
    PAL_CNTL.iter()
        .flat_map(|p: &Palette| p.entries
        .iter()
        .map(|e: &PaletteEntry| Color::RGBA(e.r,e.g,e.b,e.a)))
        .collect()
}

pub fn get_palette(number: u8) -> &'static Palette {
    &PAL_CNTL[number as usize]
}

lazy_static! {
	static ref PAL_CNTL:Vec<Palette> = vec![
		// #0
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
			]
		},
		// #1
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #2
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
			]
		},
		// #3
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x12, g: 0xef, b: 0x11, a: 0xff},
			]
		},
		// #4
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
			]
		},
		// #5
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xe2, g: 0x0c, b: 0xeb, a: 0xff},
				PaletteEntry {r: 0x1a, g: 0xf9, b: 0xf8, a: 0xff},
			]
		},
		// #6
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
			]
		},
		// #7
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #8
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #9
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #10
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #11
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
			]
		},
		// #12
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #13
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #14
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #15
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
			]
		},
		// #16
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
			]
		},
		// #17
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #18
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
			]
		},
		// #19
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #20
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #21
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #22
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #23
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #24
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #25
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #26
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #27
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
			]
		},
		// #28
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #29
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #30
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #31
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
			]
		},
		// #32
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
			]
		},
		// #33
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #34
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
			]
		},
		// #35
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec, a: 0xff},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
			]
		},
		// #36
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec, a: 0xff},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
			]
		},
		// #37
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec, a: 0xff},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
			]
		},
		// #38
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec, a: 0xff},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
			]
		},
		// #39
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #40
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #41
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #42
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #43
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
			]
		},
		// #44
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #45
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #46
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #47
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
			]
		},
		// #48
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
			]
		},
		// #49
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #50
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
			]
		},
		// #51
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
			]
		},
		// #52
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
			]
		},
		// #53
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
			]
		},
		// #54
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
			]
		},
		// #55
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
			]
		},
		// #56
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #57
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #58
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0, a: 0xff},
			]
		},
		// #59
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
			]
		},
		// #60
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe, a: 0xff},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #61
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8, a: 0xff},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4, a: 0xff},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee, a: 0xff},
			]
		},
		// #62
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6, a: 0xff},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5, a: 0xff},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04, a: 0xff},
			]
		},
		// #63
		Palette {
			entries: [
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00, a: 0xff},
			]
		},
	];
}




