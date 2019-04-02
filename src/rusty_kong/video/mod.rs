mod common;

use self::common::*;

mod palettes;

use self::palettes::get_palette;

mod sprites;

use self::sprites::get_sprite_bitmap;

mod tiles;

use self::tiles::get_tile_bitmap;

use std::vec::Vec;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

lazy_static! { // Macro que crea variables static
    static ref SPR_CNTL: Vec<SpriteControlBlock>=vec!();
    static ref BG1_CNTL: Vec<BackgroundControlBlock>=vec!();
}

fn video_bg(canvas: &mut WindowCanvas) {}

fn video_fg(canvas: &mut WindowCanvas) {}

pub fn video_update(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    video_bg(canvas);
    video_fg(canvas);
    canvas.present();
}

pub fn video_init(sdl_context: &Sdl) -> WindowCanvas {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
        "Rusty Kong",
        SCREEN_WIDTH * 3,
        SCREEN_HEIGHT * 3)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    return window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
}