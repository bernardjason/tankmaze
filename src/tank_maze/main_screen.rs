extern crate pretty_env_logger;
extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, BlendMode};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;


use self::sdl2::render::{Canvas, Texture};
use self::sdl2::video::Window;

use crate::tank_maze::player_car::{PlayerCar, get_rotated};
use crate::tank_maze::maze::{Maze, CELL_SIZE};
use sdl2::pixels::Color;
use crate::tank_maze::{common, tank_maze};
use sdl2::rect::{Point, Rect};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use crate::tank_maze::common::{SCREEN_WIDTH, SCREEN_HEIGHT, Event, make_title_texture};
use sdl2::render::BlendMode::Blend;
use std::time::{Duration, SystemTime};
use crate::tank_maze::sound::{play, HIT_WALL, stop, ENGINE};
use crate::tank_maze::extra_prizes::Prize_Type;


const BANNER_HEIGHT: i32 = 100;
const LIVES: i32 = 2;

pub(crate) struct MainScreen<'a> {
    player_texture: Texture<'a>,
    prize_texture: Texture<'a>,
    extra_textures:[Texture<'a>;2],
    player: PlayerCar,
    maze: Maze,
    prize_size: u32,
    level: u32,
    pub level_done: bool,
    click_counter: i32,
    well_done: Texture<'a>,
    oh_dear: Texture<'a>,
    start_time: SystemTime,
    pub lives:i32,
    pub took: i64,
    game_over:bool,
    pub bonus_points:i64,

}

impl<'a> MainScreen<'a> {
    pub(crate) fn new(texture_creator: &'a TextureCreator<WindowContext>, font: &Font) -> MainScreen<'a> {
        let player = texture_creator.load_texture("artifacts/car.png").unwrap();
        let prize_texture = texture_creator.load_texture("artifacts/prize.png").unwrap();
        let extras = [
            texture_creator.load_texture("artifacts/extra_projectile.png").unwrap(),
            texture_creator.load_texture("artifacts/extra_time.png").unwrap(),
        ];
        let player_car = PlayerCar::new();

        let maze = Maze::new(1);

        let get_pixels_surface1 = font.render("Well done").blended(Color::WHITE).map_err(|e| e.to_string()).unwrap();
        let get_pixels_surface2 = font.render("Oh dear").blended(Color::WHITE).map_err(|e| e.to_string()).unwrap();
        let well_done = make_title_texture(BANNER_HEIGHT as u32,
                                           Color::MAGENTA,
                                           &texture_creator,
                                           get_pixels_surface1);
        let oh_dear = make_title_texture(BANNER_HEIGHT as u32,
                                         Color::YELLOW,
                                         &texture_creator,
                                         get_pixels_surface2);

        MainScreen {
            player_texture: player,
            prize_texture: prize_texture,
            extra_textures:extras,
            player: player_car,
            maze: maze,
            prize_size: (CELL_SIZE / 2) as u32,
            level: 1,
            level_done: false,
            click_counter: 0,
            well_done: well_done,
            oh_dear: oh_dear,
            start_time: SystemTime::now(),
            lives:LIVES,
            took: 0,
            game_over:false,
            bonus_points:0,
        }
    }
    pub fn new_game(mut self) -> MainScreen<'a> {
        self.level = 0;
        self.lives = LIVES;
        self.bonus_points=0;
        self.new_level();
        return self;
    }

    fn new_level(&mut self) {
        let left_over_projectiles = self.player.available_projectiles;
        self.player = PlayerCar::new();
        self.player.available_projectiles = self.player.available_projectiles + left_over_projectiles;
        if self.player.available_projectiles > PlayerCar::MAX_PROJECTILES {
            self.player.available_projectiles = PlayerCar::MAX_PROJECTILES;
        }
        self.click_counter = 0;
        self.level_done = false;
        self.game_over = false;
        if measure_against_estimate(self.maze.squares, self.took) >= 0 {
            self.level = self.level + 1;
        }
        self.maze = Maze::new(self.level);
        self.start_time = SystemTime::now();
    }

    pub fn update(&mut self, current_event: &common::Event) -> bool {
        if self.level_done {
            self.click_counter = self.click_counter + 1;
            if self.lives <= 0 {
                self.game_over = true;
                self.level_done = true;
            }
            stop(ENGINE);

            if *current_event == Event::Space || self.click_counter > (SCREEN_WIDTH / 2) as i32 {
                if self.game_over {
                   return false;
                }
                self.new_level();
            }
        } else {
            self.playing_update(current_event);
        }
        return true;
    }

    fn playing_update(&mut self, current_event: &Event) {
        let previous = self.player.update(current_event);
        self.took = self.start_time.elapsed().unwrap().as_secs() as i64 - self.player.bonus_time;
        let player_points = get_rotated(&self.player);
        if self.maze.collision(&player_points) == true {
            self.player.rollback(previous);
            play(HIT_WALL);
        }

        let mut i = self.player.projectiles.len() as i32 - 1;
        while i >= 0 {
            let p = self.player.projectiles.get(i as usize).unwrap();
            if self.maze.collision_remove_wall(p.point.as_ref()) == true {
                self.player.projectiles.remove(i as usize);
            }
            i = i - 1;
        }
        let x1 = self.player.x;
        let y1 = self.player.y;
        let x2 = self.maze.end_x_y.0 as f64;
        let y2 = self.maze.end_x_y.1 as f64;

        let distance = ((y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1)).sqrt();
        if distance < self.prize_size as f64 * 0.75 {
            self.level_done = true;
            stop(ENGINE);
            let estimate_over = measure_against_estimate(self.maze.squares, self.took);
            if estimate_over < 0 {
                self.lives = self.lives - 1;
            } else {
                self.bonus_points = self.bonus_points + (estimate_over * 100) as i64;
            }
        }
        for extra in self.maze.bonus_items.iter_mut() {
            if extra.still_valid {
                let centre_x = extra.collide.rect.width() / 1;
                let centre_y = extra.collide.rect.height() / 1;
                let radius = centre_x + centre_y / 2;
                let x2 = extra.x + centre_x as f64;
                let y2 = extra.y + centre_y as f64;

                let distance = ((y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1)).sqrt();
                if distance < radius as f64 * 0.55 {
                    extra.still_valid = false;

                    match extra.type_of_prize {
                        Prize_Type::EXTRA_PROJECTILES => {
                            self.player.available_projectiles = self.player.available_projectiles + 5;
                        }
                        Prize_Type::EXTRA_TIME => {
                            self.player.bonus_time = self.player.bonus_time + 10;
                        }
                    }
                }
            }
        }
    }

    pub fn draw_on_canvas(self, canvas: &mut Canvas<Window>, font: &Font, texture_creator: &'a TextureCreator<WindowContext>) -> MainScreen<'a> {
        if self.level_done {
            let took = self.took;
            let clicker = self.click_counter;
            let banner_y = 400;
            let mut estimate_over = measure_against_estimate(self.maze.squares, self.took);

            let me = self.playing_draw_in_canvas(canvas);

            let mut adj = Rect::new(me.click_counter, banner_y, SCREEN_WIDTH, BANNER_HEIGHT as u32);
            canvas.copy(&me.well_done, None, adj).unwrap();
            if clicker % 20 < 14 {
                let mut score_text = format!("Took seconds {} quicker by {} bonus now {}", took, estimate_over,me.bonus_points);
                if estimate_over < 0 {
                    estimate_over = estimate_over * -1;
                    score_text = format!("Took seconds {} but too slow by {} seconds", took, estimate_over);
                }
                let font_surface = font.render(score_text.as_str()).blended(Color::YELLOW).unwrap();
                let mut texture_text = font_surface.as_texture(&texture_creator).unwrap();
                texture_text.set_blend_mode(BlendMode::Blend);
                canvas.copy(&texture_text, None, Rect::new(me.click_counter, banner_y + BANNER_HEIGHT, font_surface.width(), 48));
            }
            me
        } else {
            let seconds = self.start_time.elapsed().unwrap().as_secs() as i64 - self.player.bonus_time ;
            let mut estimate_over = measure_against_estimate(self.maze.squares, self.took);
            let mut score_text = format!("Lives {} Level {} Projectiles {}    seconds {} quicker by {}", self.lives,self.level, self.player.available_projectiles, seconds, estimate_over);
            if estimate_over < 0 {
                estimate_over = estimate_over * -1;
                score_text = format!("Lives {} Level {} Projectiles {}      seconds {} too slow by {}", self.lives,self.level, self.player.available_projectiles, seconds, estimate_over);
            }

            let font_surface = font.render(score_text.as_str()).blended(Color::YELLOW).unwrap();
            let mut texture_text = font_surface.as_texture(&texture_creator).unwrap();
            texture_text.set_blend_mode(BlendMode::Blend);
            canvas.copy(&texture_text, None, Rect::new(0, 0, font_surface.width(), 48));


            self.playing_draw_in_canvas(canvas)
        }
    }

    fn playing_draw_in_canvas(self, canvas: &mut Canvas<Window>) -> MainScreen<'a> {
        let mut adj = self.player.collide.rect.clone();
        adj.set_x((SCREEN_WIDTH / 2) as i32);
        adj.set_y((SCREEN_HEIGHT / 2) as i32);
        adj.offset((self.player.collide.rect.width() as i32 / -2), (self.player.collide.rect.height() as i32 / -2));

        canvas.copy_ex(&self.player_texture, None,
                       Some(adj),
                       self.player.rotate, None, false, false).unwrap();

        canvas.set_draw_color(Color::CYAN);
        for w in self.maze.maze_object.iter() {
            if w.visible {
                let mut adj = w.collide.rect.clone();

                adj.offset(((SCREEN_WIDTH as i32 / 2) - self.player.x.round() as i32),
                           (SCREEN_HEIGHT as i32 / 2) - self.player.y.round() as i32);
                canvas.draw_rect(adj).unwrap();
            }
        }

        canvas.set_draw_color(Color::GREEN);
        for p in self.player.projectiles.iter() {
            let mut adj = p.collide.rect.clone();

            adj.offset(((SCREEN_WIDTH as i32 / 2) - self.player.x.round() as i32),
                       (SCREEN_HEIGHT as i32 / 2) - self.player.y.round() as i32);
            canvas.fill_rect(adj).unwrap();
        }

        for extra in self.maze.bonus_items.iter() {
            if extra.still_valid {
                let mut adj = extra.collide.rect.clone();
                adj.offset((extra.collide.rect.width() as i32 / 2), (extra.collide.rect.height() as i32 / 2));
                adj.offset(((SCREEN_WIDTH as i32 / 2) - self.player.x.round() as i32),
                           (SCREEN_HEIGHT as i32 / 2) - self.player.y.round() as i32);
                match extra.type_of_prize {
                    Prize_Type::EXTRA_PROJECTILES => {
                        canvas.copy(&self.extra_textures[0], None, adj).unwrap();
                    }
                    Prize_Type::EXTRA_TIME => {
                        canvas.copy(&self.extra_textures[1], None, adj).unwrap();
                    }
                }
            }
        }


        let mut adj = Rect::new(self.maze.end_x_y.0, self.maze.end_x_y.1, self.prize_size, self.prize_size);
        adj.offset((self.prize_size as i32 / -2), (self.prize_size as i32 / -2));
        adj.offset(((SCREEN_WIDTH as i32 / 2) - self.player.x.round() as i32),
                   (SCREEN_HEIGHT as i32 / 2) - self.player.y.round() as i32);
        canvas.copy(&self.prize_texture, None, adj).unwrap();

        self
    }


    pub fn car_moves_on_screen_draw_on_canvas(self, canvas: &mut Canvas<Window>) -> MainScreen<'a> {
        let mut adj = self.player.collide.rect.clone();
        adj.offset((self.player.collide.rect.width() as i32 / -2), (self.player.collide.rect.height() as i32 / -2));

        canvas.copy_ex(&self.player_texture, None,
                       Some(adj),
                       self.player.rotate, None, false, false).unwrap();

        canvas.set_draw_color(Color::CYAN);
        for w in self.maze.maze_object.iter() {
            let mut adj = w.collide.rect.clone();
            adj.offset((w.collide.rect.width() as i32 / -2) as i32, (w.collide.rect.height() as i32 / -2) as i32);
            canvas.draw_rect(w.collide.rect).unwrap();
        }

        self
    }
}

fn measure_against_estimate(squares: u32, took: i64) -> i32 {
    return (squares as f64 * 0.35 - took as f64) as i32;
}
