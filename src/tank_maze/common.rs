extern crate pretty_env_logger;
extern crate sdl2;

use sdl2::rect::Rect;

use self::sdl2::render::{Canvas, Texture};
use self::sdl2::video::{Window};
use sdl2::render::{TextureCreator, BlendMode};
use sdl2::video::WindowContext;
use sdl2::ttf::Font;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use sdl2::render::BlendMode::Blend;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub const SCREEN_WIDTH: u32 = 1024;
pub const SCREEN_HEIGHT: u32 = 800;

#[derive(PartialEq, Eq)]
pub enum Event {
    None,
    Up,
    Down,
    Left,
    Right,
    Shift,
    Space,
    off_Up,
    off_Down,
    off_Left,
    off_Right,
    off_Shift,
    off_Space,
}

pub fn make_title_texture<'a>(height: u32, colour: Color,
                              texture_creator: &'a TextureCreator<WindowContext>, get_pixels_surface: Surface) -> Texture<'a> {
    let size = get_pixels_surface.width() * get_pixels_surface.height();
    let mut pixel_buffer = Vec::with_capacity(size as usize);
    pixel_buffer.resize(size as usize, 0);

    let off_top = 11;
    get_pixels_surface.with_lock(|buffer: &[u8]| {
        for y in off_top..get_pixels_surface.height() {
            for x in 0..get_pixels_surface.width() {
                let index = (y * get_pixels_surface.pitch() + x * 4) as usize;
                let val = buffer[index + 3];
                if val > 0 {
                    let index = ((y - off_top) * get_pixels_surface.width() + x) as usize;
                    pixel_buffer[index] = 1;
                }
            }
        }
    }
    );

    let mut title_by: Texture =
        texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, SCREEN_WIDTH as u32, height).expect("texture");
    title_by.set_blend_mode(BlendMode::Blend);
        title_by.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let gap = 4;
            let mut slant = 0;
            for oppy in 1..get_pixels_surface.height() {
                let y = get_pixels_surface.height() - oppy - 1;
                for x in 0..get_pixels_surface.width() {
                    let index = y * get_pixels_surface.width() + x;
                    let val = pixel_buffer[index as usize];
                    if val == 1 {
                        let size = gap / 2;
                        for yy in y * gap..y * gap + size {
                            for xx in x * gap..x * gap + size {
                                let offset = ((yy) * pitch as u32 + (xx + slant) * 4) as usize;
                                buffer[offset] = 255;
                                buffer[offset + 1] = colour.r;
                                buffer[offset + 2] = colour.g;
                                buffer[offset + 3] = colour.b;
                            }
                        }
                    }
                }
                slant = slant + 2;
            }
        }).unwrap();

    title_by
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
