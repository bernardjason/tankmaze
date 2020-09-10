extern crate pretty_env_logger;
extern crate sdl2;

use sdl2::rect::Rect;

use self::sdl2::render::{Canvas, Texture};
use self::sdl2::video::{Window};
use crate::tank_maze::common::{SCREEN_WIDTH, make_title_texture};
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::ttf::Font;
use sdl2::pixels::{Color};
use sdl2::image::LoadTexture;

pub(crate) struct StartScreen<'a> {
    bernie_soft_title: Texture<'a>,
    car_maze_title: Texture<'a>,
    instructions: Vec<&'a str>,
    extras: [Texture<'a>;2],
    pub(crate) bernie_x: i32,
}

impl<'a> StartScreen<'a> {
    pub(crate) fn new(texture_creator: &'a TextureCreator<WindowContext>, font: &Font) -> StartScreen<'a> {
        let get_pixels_surface1 = font.render("Berniesoft").blended(Color::WHITE).map_err(|e| e.to_string()).unwrap();
        let get_pixels_surface2 = font.render("Tank Maze...").blended(Color::WHITE).map_err(|e| e.to_string()).unwrap();
        let title1 = make_title_texture(200,
                                        Color::YELLOW,
                                        &texture_creator,
                                        get_pixels_surface1);
        let title2 = make_title_texture(200,
                                        Color::GREEN,
                                        &texture_creator,
                                        get_pixels_surface2);
        let extras = [
            texture_creator.load_texture("artifacts/extra_projectile.png").unwrap(),
            texture_creator.load_texture("artifacts/extra_time.png").unwrap(),
        ];
        let instructions = vec![
            "Find your way to the red star at bottom right of the maze.",
            "Left and right arrow keys to rotate.",
            "Up arrow go faster. Left shift slow down. Space fire",
            "Too slow and you will loose a life.",
            "Hit the maze and you will loose a life.",
            "You have limited projectiles to blow a way through the maze.",
            "You can get extra time with spiral bonus.",
            "You can get extra projectiles with star bonus",
        ];
        StartScreen {
            bernie_soft_title: title1,
            car_maze_title: title2,
            instructions: instructions,
            extras:extras,
            bernie_x: 0,
        }
    }
    pub fn update(&mut self) {
        self.bernie_x = self.bernie_x + 1;
    }
    pub fn draw_on_canvas(self, canvas: &mut Canvas<Window>, font: &Font, texture_creator: &'a TextureCreator<WindowContext>) -> StartScreen<'a> {
        canvas.copy(&self.bernie_soft_title, None, Some(Rect::new(self.bernie_x, 100, SCREEN_WIDTH, 200))).unwrap();
        canvas.copy(&self.car_maze_title, None, Some(Rect::new(0, 300, SCREEN_WIDTH, 200))).unwrap();

        let mut y = 400;
        for line in self.instructions.iter() {
            let font_surface = font.render(line).blended(Color::YELLOW).unwrap();
            let mut instructions_texture = font_surface.as_texture(&texture_creator).unwrap();

            canvas.copy(&instructions_texture, None, Some(Rect::new(10, y, font_surface.width(), 48))).unwrap();
            y = y + 48;
        }
        canvas.copy(&self.extras[1], None, Some(Rect::new((SCREEN_WIDTH - 350) as i32, 690, 48, 48))).unwrap();
        canvas.copy(&self.extras[0], None, Some(Rect::new((SCREEN_WIDTH - 350) as i32, 740, 48, 48))).unwrap();

        self
    }
}
