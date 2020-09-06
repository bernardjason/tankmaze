extern crate pretty_env_logger;
extern crate sdl2;

use sdl2::rect::Rect;

use self::sdl2::render::{Canvas, Texture};
use self::sdl2::video::{Window};
use crate::tank_maze::common::{SCREEN_WIDTH, make_title_texture};
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::ttf::Font;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;

pub(crate) struct StartScreen<'a> {
    bernie_soft_title: Texture<'a>,
    car_maze_title: Texture<'a>,
    pub(crate) bernie_x:i32
}

impl <'a> StartScreen<'a> {
    pub(crate) fn new(texture_creator: &'a TextureCreator<WindowContext>, font: &Font) -> StartScreen<'a> {
        let get_pixels_surface1 = font.render("Berniesoft").blended(Color::WHITE).map_err(|e| e.to_string()).unwrap();
        let get_pixels_surface2 = font.render("Carnage Maze").blended(Color::WHITE).map_err(|e| e.to_string()).unwrap();
        let title1 = make_title_texture(200,
                                       Color::YELLOW,
                                       &texture_creator,
                                       get_pixels_surface1);
        let title2 = make_title_texture(200,
                                        Color::GREEN,
                                        &texture_creator,
                                        get_pixels_surface2);
        StartScreen {
            bernie_soft_title: title1,
            car_maze_title:title2,
            bernie_x:0
        }
    }
    pub fn update(&mut self){
        self.bernie_x = self.bernie_x+1;
    }
    pub fn draw_on_canvas(self, canvas:&mut Canvas<Window>) -> StartScreen<'a> {
        canvas.copy(&self.bernie_soft_title, None, Some(Rect::new(self.bernie_x, 100, SCREEN_WIDTH, 200))).unwrap();
        canvas.copy(&self.car_maze_title, None, Some(Rect::new(0, 300, SCREEN_WIDTH, 200))).unwrap();
        self
    }
}
pub fn xxxxmake_title_texture<'a>(height: u32, colour: Color,
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
        texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH as u32, height).expect("texture");
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
                            let offset = ((yy) * pitch as u32 + (xx + slant) * 3) as usize;
                            buffer[offset] = colour.r;
                            buffer[offset + 1] = colour.g;
                            buffer[offset + 2] = colour.b;
                        }
                    }
                }
            }
            slant = slant + 2;
        }
    }).unwrap();
    title_by
}
