use std::cell::RefCell;

use super::sprites::get_sprite_bitmap;
use super::tiles::get_tile_bitmap;
use super::palettes::get_palette_colors;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Palette as SdlPalette;
use sdl2::render::WindowCanvas;

pub const SCREEN_WIDTH: u32 = 256;
pub const SCREEN_HEIGHT: u32 = 256;

pub const TILE_WIDTH: u32 = 8;
pub const TILE_HEIGHT: u32 = 8;
pub const TILE_MAX: u32 = 256;
pub const TILE_COL_COUNT: u32 = 32;
pub const TILE_ROW_COUNT: u32 = 32;
pub const TILE_CNTL_MAX: u32 = TILE_ROW_COUNT * TILE_COL_COUNT;

pub const SPRITE_WIDTH: u32 = 16;
pub const SPRITE_HEIGHT: u32 = 16;
pub const SPRITE_MAX: u32 = 128;

pub const F_SPR_NONE: u8 = 0b00000000;
pub const F_SPR_ENABLED: u8 = 0b00000001;
pub const F_SPR_COLLIDED: u8 = 0b00000010;
pub const F_SPR_HFLIP: u8 = 0b00000100;
pub const F_SPR_VFLIP: u8 = 0b00001000;
pub const F_SPR_CHANGED: u8 = 0b00010000;

pub const F_BG_NONE: u8 = 0b00000000;
pub const F_BG_ENABLED: u8 = 0b00000001;
pub const F_BG_HFLIP: u8 = 0b00000010;
pub const F_BG_VFLIP: u8 = 0b00000100;
pub const F_BG_CHANGED: u8 = 0b00001000;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TileMaps {
    LongIntroduction,
    Level1,
}

#[derive(Copy, Clone)]
pub struct Palette {
    pub entries: [PaletteEntry; 4],
}

#[derive(Copy, Clone)]
pub struct PaletteEntry {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct TileMapEntry(pub u16, pub u8, pub u8);

#[derive(Copy, Clone)]
pub struct TileMap {
    pub entries: [TileMapEntry; (TILE_ROW_COUNT * TILE_COL_COUNT) as usize]
}

#[derive(Copy, Clone)]
pub struct SpriteControlBlock {
    y: u16,
    x: u16,
    flags: u8,
    tile: u16,
    palette: u8,
    user_data1: u32,
    user_data2: u32,
}

pub struct SpriteControlTable {
    table: [SpriteControlBlock; SPRITE_MAX as usize],
    surfaces: Vec<RefCell<Surface<'static>>>,
}

impl SpriteControlTable {
    pub fn new() -> SpriteControlTable {
        let mut table = SpriteControlTable {
            surfaces: vec![],
            table: [SpriteControlBlock::new_empty(); SPRITE_MAX as usize],
        };

        for _i in 0..SPRITE_MAX {
            let mut surface = Surface::new(
                16,
                16,
                PixelFormatEnum::Index8)
                .unwrap();
            let palette = SdlPalette::with_colors(&get_palette_colors()).unwrap();

            surface.enable_RLE();
            surface.set_palette(&palette).unwrap();
            surface.set_color_key(true, Color::RGBA(0, 0, 0, 0xff)).unwrap();
            table.surfaces.push(RefCell::new(surface));
        }
        table
    }

    pub fn update(&mut self, canvas: &mut WindowCanvas) {
        let mut sprite_number: usize = 0;
        let texture_creator = canvas.texture_creator();

        for block in self.table.iter_mut() {
            let mut surface = self.surfaces[sprite_number].borrow_mut();
            if block.is_changed() {
                let spr_bitmap = get_sprite_bitmap(block.tile);
                let palette_offset = block.palette * 4;
                surface.with_lock_mut(|pixels: &mut [u8]| {
                    for i in 0..256 {
                        pixels[i] = spr_bitmap[i] + palette_offset;
                    }

                    if block.is_horizontally_flipped() {
                        let mut sx = 15;
                        let mut sy = 0;
                        for y in 0..16 {
                            for x in 0..8 {
                                let temp = pixels[y * 16 + x];
                                pixels[y * 16 + x] = pixels[sy * 16 + sx];
                                pixels[sy * 16 + sx] = temp;

                                sx -= 1;
                            }
                            sx = 15;
                            sy += 1;
                        }
                    }

                    if block.is_vertically_flipped() {
                        let mut sx = 0;
                        let mut sy = 15;
                        for y in 0..8 {
                            for x in 0..16 {
                                let temp = pixels[y * 16 + x];
                                pixels[y * 16 + x] = pixels[sy * 16 + sx];
                                pixels[sy * 16 + sx] = temp;

                                sx += 1;
                            }
                            sx = 0;
                            sy -= 1;
                        }
                    }
                });
                block.changed(false);
            }

            if block.is_enabled() {
                let sprite_texture = texture_creator
                    .create_texture_from_surface(&*surface)
                    .unwrap();
                canvas.copy(
                    &sprite_texture,
                    None,
                    Rect::new(block.x as i32, block.y as i32, 16 * 4, 16 * 4))
                    .unwrap();
            }


            sprite_number += 1;
        }
    }

    pub fn sprite(&mut self, number: u8, x: u16, y: u16, tile: u16, palette: u8, flags: u8) {
        let block = &mut self.table[number as usize];
        block.x = x;
        block.y = y;
        block.tile = tile;
        block.palette = palette;
        block.flags = flags | F_SPR_CHANGED;
    }
}

impl<'a> SpriteControlBlock {
    pub fn new_empty() -> SpriteControlBlock {
        SpriteControlBlock {
            y: 0,
            x: 0,
            tile: 0,
            palette: 0,
            flags: F_SPR_NONE,
            user_data1: 0,
            user_data2: 0,
        }
    }

//    pub fn update(self: &SpriteControlBlock) {
//        use super::palettes::get_palette;
//        use super::sprites::get_sprite_bitmap;
//    }

    pub fn tile(self: &mut SpriteControlBlock, number: u16) {
        self.tile = number;
    }

    pub fn is_changed(self: &SpriteControlBlock) -> bool {
        self.flags & F_SPR_CHANGED != 0
    }

    pub fn is_enabled(self: &SpriteControlBlock) -> bool {
        self.flags & F_SPR_ENABLED != 0
    }

    pub fn is_collided(self: &mut SpriteControlBlock) -> bool {
        self.flags & F_SPR_COLLIDED != 0
    }

    pub fn enable(self: &mut SpriteControlBlock, flag: bool) {
        if flag {
            self.flags |= F_SPR_ENABLED;
        } else {
            self.flags &= !F_SPR_ENABLED;
        }
    }

    pub fn changed(self: &mut SpriteControlBlock, flag: bool) {
        if flag {
            self.flags |= F_SPR_CHANGED;
        } else {
            self.flags &= !F_SPR_CHANGED;
        }
    }

    pub fn collided(self: &mut SpriteControlBlock, flag: bool) {
        if flag {
            self.flags |= F_SPR_COLLIDED;
        } else {
            self.flags &= !F_SPR_COLLIDED;
        }
    }

    pub fn palette(self: &mut SpriteControlBlock, number: u8) {
        self.palette = number;
    }

    pub fn get_position(self: &SpriteControlBlock) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn position(self: &mut SpriteControlBlock, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn vertical_flip(self: &mut SpriteControlBlock, flag: bool) {
        if flag {
            self.flags |= F_SPR_VFLIP;
        } else {
            self.flags &= !F_SPR_VFLIP;
        }
    }

    pub fn is_vertically_flipped(self: &SpriteControlBlock) -> bool {
        self.flags & F_SPR_VFLIP != 0
    }

    pub fn horizontal_flip(self: &mut SpriteControlBlock, flag: bool) {
        if flag {
            self.flags |= F_SPR_HFLIP;
        } else {
            self.flags &= !F_SPR_HFLIP;
        }
    }

    pub fn is_horizontally_flipped(self: &SpriteControlBlock) -> bool {
        self.flags & F_SPR_HFLIP != 0
    }
}

#[derive(Copy, Clone)]
pub struct BackgroundControlBlock {
    tile: u16,
    flags: u8,
    palette: u8,
    user_data1: u32,
    user_data2: u32,
}

pub struct BackgroundControlTable {
    table: [BackgroundControlBlock; TILE_CNTL_MAX as usize],
    surfaces: Vec<RefCell<Surface<'static>>>,
}

impl BackgroundControlTable {
    pub fn new() -> BackgroundControlTable {
        let mut table = BackgroundControlTable {
            surfaces: vec![],
            table: [BackgroundControlBlock::new_empty(); TILE_CNTL_MAX as usize],
        };

        for _i in 0..TILE_CNTL_MAX {
            let mut surface = Surface::new(
                8,
                8,
                PixelFormatEnum::Index8)
                .unwrap();
            let palette = SdlPalette::with_colors(&get_palette_colors()).unwrap();
            surface.set_palette(&palette).unwrap();
            table.surfaces.push(RefCell::new(surface));
        }
        table
    }

    pub fn update(&mut self, bg_surface: &mut Surface) {
        let mut block_number: usize = 0;
        let mut tile_rect = Rect::new(0, 0, 8, 8);

        for block in self.table.iter_mut() {
            if block.is_changed() && block.is_enabled() {
                let tile_bitmap = get_tile_bitmap(block.tile);
                let mut surface = self.surfaces[block_number].borrow_mut();
                let palette_offset = block.palette * 4;
                surface.with_lock_mut(|pixels: &mut [u8]| {
                    for i in 0..64 {
                        pixels[i] = tile_bitmap[i] + palette_offset;
                    }

                    if block.is_horizontally_flipped() {
                        let mut sx = 7;
                        let mut sy = 0;
                        for y in 0..8 {
                            for x in 0..4 {
                                let temp = pixels[y * 8 + x];
                                pixels[y * 8 + x] = pixels[sy * 8 + sx];
                                pixels[sy * 8 + sx] = temp;

                                sx -= 1;
                            }
                            sx = 7;
                            sy += 1;
                        }
                    }

                    if block.is_vertically_flipped() {
                        let mut sx = 0;
                        let mut sy = 7;
                        for y in 0..4 {
                            for x in 0..8 {
                                let temp = pixels[y * 8 + x];
                                pixels[y * 8 + x] = pixels[sy * 8 + sx];
                                pixels[sy * 8 + sx] = temp;

                                sx += 1;
                            }
                            sx = 0;
                            sy -= 1;
                        }
                    }
                });
                block.changed(false);
                surface.blit(None, bg_surface, tile_rect).unwrap();
            }

            tile_rect.offset(8, 0);
            if tile_rect.x() == 256 {
                tile_rect.offset(-256, 8);
            }

            block_number += 1;
        }
    }

    pub fn set(&mut self, tile_map: TileMaps) {
        use super::tile_maps::INTRO_MAP;

        match tile_map {
            TileMaps::LongIntroduction => {
                let mut index = 0;
                for entry in INTRO_MAP.iter() {
                    let mut block = &mut self.table[index];
                    block.enable(true);
                    //block.changed(true);
                    block.tile(entry.0);
                    block.palette(entry.1);
                    block.vertical_flip(entry.2 & F_BG_VFLIP != 0);
                    block.horizontal_flip(entry.2 & F_BG_HFLIP != 0);

                    index += 1;
                }
            }
            _ => {}
        }
    }
}

impl BackgroundControlBlock {
    pub fn new_empty() -> BackgroundControlBlock {
        BackgroundControlBlock {
            tile: 0,
            flags: F_BG_NONE,
            palette: 0,
            user_data1: 0,
            user_data2: 0,
        }
    }

    pub fn is_changed(self: &BackgroundControlBlock) -> bool {
        self.flags & F_BG_CHANGED != 0
    }

    pub fn is_enabled(self: &BackgroundControlBlock) -> bool {
        self.flags & F_BG_ENABLED != 0
    }

    pub fn tile(self: &mut BackgroundControlBlock, number: u16) {
        self.tile = number;
        self.changed(true);
    }

    pub fn changed(self: &mut BackgroundControlBlock, flag: bool) {
        if flag {
            self.flags |= F_BG_CHANGED;
        } else {
            self.flags &= !F_BG_CHANGED;
        }
    }

    pub fn palette(self: &mut BackgroundControlBlock, number: u8) {
        self.palette = number;
        self.changed(true);
    }

    pub fn enable(self: &mut BackgroundControlBlock, flag: bool) {
        if flag {
            self.flags |= F_BG_ENABLED;
        } else {
            self.flags &= !F_BG_ENABLED;
        }
    }

    pub fn vertical_flip(self: &mut BackgroundControlBlock, flag: bool) {
        if flag {
            self.flags |= F_BG_VFLIP;
        } else {
            self.flags &= !F_BG_VFLIP;
        }
        self.changed(true);
    }

    pub fn is_vertically_flipped(self: &BackgroundControlBlock) -> bool {
        self.flags & F_BG_VFLIP != 0
    }

    pub fn horizontal_flip(self: &mut BackgroundControlBlock, flag: bool) {
        if flag {
            self.flags |= F_BG_HFLIP;
        } else {
            self.flags &= !F_BG_HFLIP;
        }
        self.changed(true);
    }

    pub fn is_horizontally_flipped(self: &BackgroundControlBlock) -> bool {
        self.flags & F_BG_HFLIP != 0
    }
}