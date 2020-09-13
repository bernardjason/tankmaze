extern crate pretty_env_logger;
extern crate sdl2;

use std::fs::File;
use std::io::{ Write};
use std::path::Path;

use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::tank_maze::common::{make_title_texture, read_lines, SCREEN_WIDTH};

use self::sdl2::render::{Canvas, Texture};
use self::sdl2::video::Window;

struct ScoreEntry {
    score: i64,
    who: String,
}

impl ScoreEntry {
    pub fn new(score: i64, who: String) -> ScoreEntry {
        ScoreEntry {
            score: score,
            who: who,
        }
    }
}

pub(crate) struct GameEndScreen<'a> {
    bernie_soft_title: Texture<'a>,
    bernie_x: i32,
    bonus_points: i64,
    table_updated: bool,
    pub put_on_table:bool,
    table: Vec<ScoreEntry>,
}

impl<'a> GameEndScreen<'a> {
    pub(crate) fn new(texture_creator: &'a TextureCreator<WindowContext>, font: &Font) -> GameEndScreen<'a> {
        let get_pixels_surface1 = font.render("Game over").blended(Color::WHITE).map_err(|e| e.to_string()).unwrap();
        let title1 = make_title_texture(200,
                                        Color::YELLOW,
                                        &texture_creator,
                                        get_pixels_surface1);
        GameEndScreen {
            bernie_soft_title: title1,
            bernie_x: 0,
            bonus_points: 0,
            table_updated: false,
            put_on_table:false,
            table: vec![],
        }
    }
    pub fn change_to_screen(&mut self, bonus_points: i64) {
        self.bonus_points = bonus_points;
        self.bernie_x = 0;
        self.put_on_table = false;
        self.table_updated = false;
        self.get_high_scores();
        self.table.sort_by(|a, b| b.score.cmp(&a.score));
        if self.table.len() == 0 || self.table.len() < 5 {
            self.put_on_table=true;
        }
        for s in self.table.iter() {
            if s.score < bonus_points {
                self.put_on_table=true;
            }
        }
    }
    fn get_high_scores(&mut self) {
        if let Ok(lines) = read_lines("high_scores.txt") {
            self.table.clear();
            for line in lines {
                if let Ok(score) = line {
                    let fields: Vec<&str> = score.split(",").collect();
                    let entry: ScoreEntry = ScoreEntry::new(fields[0].parse::<i64>().unwrap(), fields[1].parse().unwrap());
                    self.table.push(entry);
                }
            }
        }
    }
    pub fn update(&mut self, current_input: String, input_done: bool) {
        self.bernie_x = self.bernie_x + 1;
        if input_done && self.table_updated == false {
            self.table_updated = true;
            self.table.push( ScoreEntry::new(self.bonus_points as i64, current_input));
            self.table.sort_by(|a, b| b.score.cmp(&a.score));
            for s in self.table.iter() {
                println!("{} {}", s.score, s.who);
            }
            let path = Path::new("high_scores.txt");

            if self.table.len() >= 5 {
                self.table.drain(5..);
            }
            let mut file = File::create(&path).expect("open file high_scores.xt to update");
            for s in self.table.iter() {
                let e = format!("{},{}\n",s.score,s.who);
                file.write(e.as_bytes()).unwrap();
            }

        }
    }
    pub fn draw_on_canvas(self, canvas: &mut Canvas<Window>,font:&Font,texture_creator:&'a TextureCreator<WindowContext>) -> GameEndScreen<'a> {
        canvas.copy(&self.bernie_soft_title, None, Some(Rect::new(self.bernie_x, 100, SCREEN_WIDTH, 200))).unwrap();

        let mut y = 300;
        let x = 200;
        let score_text = format!("You score was {}",self.bonus_points);
        <GameEndScreen<'a>>::draw_score_info(canvas, font, &texture_creator, y, x, score_text,Color::YELLOW);
        y = y + 48;

        let colour_list = vec![Color::MAGENTA,Color::RED,Color::GREEN,Color::CYAN];
        let mut colour_index:usize = 0;
        let mut position =1 ;
        for s in self.table.iter() {
            let score_text = format!("{:<10} {}",position,s.score);
            <GameEndScreen<'a>>::draw_score_info(canvas, font, &texture_creator, y, x, score_text,colour_list[colour_index]);

            let who = format!("{}",s.who);
            <GameEndScreen<'a>>::draw_score_info(canvas, font, &texture_creator, y, x+300, who,colour_list[colour_index]);

            y = y + 48;
            colour_index=colour_index+1;
            if colour_index >= colour_list.len() {
                colour_index=0;
            }
            position=position+1

        }



        self
    }

    fn draw_score_info(canvas: &mut Canvas<Window>, font: &Font, texture_creator: &&TextureCreator<WindowContext>, y: i32, x: i32, score_text: String,colour:Color) {
        let font_surface = font.render(score_text.as_str()).blended(colour).unwrap();
        let mut texture_text = font_surface.as_texture(&texture_creator).unwrap();
        texture_text.set_blend_mode(BlendMode::Blend);
        canvas.copy(&texture_text, None, Rect::new(x, y, font_surface.width(), 48)).unwrap();
    }
}
