extern crate sdl2;

use super::sdl2::ttf::{Sdl2TtfContext, Font};
use super::sdl2::pixels::Color;
use super::sdl2::render::{TextureCreator, Texture};
use super::sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::thread::sleep;
use std::time::Duration;
use std::borrow::{Borrow, BorrowMut};
use sdl2::render::{WindowCanvas, Canvas};
use sdl2::video::Window;

pub struct MessageArea<'tex, 'a> {
    font: &'tex Font<'tex, 'a>,
    pub input_text:String,
    initial_message:String,
    pub input_text_complete:bool,
    title:String,
    background_colour:Color,
    text_colour:Color,
    banner:String,
    cursor_flash:u64,
}

const HEIGHT:u32 = 48;

impl<'tex, 'a> MessageArea<'tex, 'a> {
    //pub fn new(font:&mut Font, message:&str, texture_creator: &'tex TextureCreator<WindowContext>) -> MessageArea<'tex> {
    pub fn new(font: &'a mut Font, initial_message: String,title:String,background_colour:Color,text_colour:Color,banner:String) -> MessageArea<'tex, 'a> {
        font.set_style(sdl2::ttf::FontStyle::NORMAL);


        MessageArea {
            font: font,
            input_text: "".parse().unwrap(),
            initial_message: initial_message,
            input_text_complete:false,
            title,
            background_colour,
            text_colour,
            banner,
            cursor_flash:0
        }
    }

    pub fn on_canvas(&mut self, canvas:&mut Canvas<Window>, x:i32, y:i32) -> &mut MessageArea<'tex, 'a> { //,keycode:Keycode,keymod:sdl2::keyboard::Mod) {

        let texture_creator: TextureCreator<_> = canvas.texture_creator();
        let mut input_width = 0;

        let input_surface = &self.font.render(self.initial_message.as_ref()).blended(self.text_colour).unwrap();
        input_width = input_surface.width();
        let mut input_texture = input_surface.as_texture(&texture_creator).unwrap();

        let mut input_text_changed = true;

        if input_text_changed {
            if self.input_text.len() > 0 {
                let input_surface = &self.font.render(self.input_text.as_str()).blended(self.text_colour).unwrap();
                input_width = input_surface.width();
                input_texture = input_surface.as_texture(&texture_creator).unwrap();
            } else {
                let mut faint = self.text_colour.clone();
                faint.a = faint.a / 3;
                let input_surface = &self.font.render(self.initial_message.as_ref()).blended(faint).unwrap();
                input_width = input_surface.width();
                input_texture = input_surface.as_texture(&texture_creator).unwrap();
            }
            input_text_changed=false;
        }

        //canvas.set_draw_color(background_colour);
        //canvas.clear();

        let input_rect = Rect::new(x, y, input_width, HEIGHT);
        canvas.copy(&input_texture, None, input_rect).unwrap();

        let cursor_offset = if self.input_text.len() > 0 {
            input_width
        }
        else {
            0
        };

        self.cursor_flash=self.cursor_flash+1;
        if self.cursor_flash % 100  < 50 {
            canvas.set_draw_color(self.text_colour);
            let cursor_rect = Rect::new(x + cursor_offset as i32, y + (HEIGHT / 4) as i32, HEIGHT / 3, (HEIGHT as f64 * 0.6) as u32);
            canvas.fill_rect(cursor_rect).unwrap();
        }

        self
    }

    pub fn key_down_handle(mut self, keycode:Keycode, keymod:sdl2::keyboard::Mod) -> MessageArea<'tex, 'a> {
        let mut name = keycode.name();
        let first = &name[..1];
        if name == "Return" {
           self.input_text_complete = true;
        } else if name == "Backspace" {
            self.input_text.pop();
        } else if name == "Space" {
            self.input_text.push(' ');
        } else if name.len() == 1 && first >= "A" && first <= "Z" {
            if keymod & sdl2::keyboard::Mod::CAPSMOD == sdl2::keyboard::Mod::CAPSMOD ||
                keymod & sdl2::keyboard::Mod::LSHIFTMOD == sdl2::keyboard::Mod::LSHIFTMOD ||
                keymod & sdl2::keyboard::Mod::RSHIFTMOD == sdl2::keyboard::Mod::RSHIFTMOD {
                ;
            } else {
                name = name.to_lowercase();
            }
            for c in name.chars() {
                self.input_text.push(c)
            }
        } else if name.len() == 1 && first >= "0" && first <= "9" {
            for c in name.chars() {
                self.input_text.push(c);
            }
        }

        self
    }

    pub fn display(self, sdl_context: &Sdl, video_subsystem: &sdl2::VideoSubsystem,
                   ) -> MessageArea<'tex, 'a> { //-> (Sdl, sdl2::VideoSubsystem) {
        let window = video_subsystem.window(self.title.as_ref(), 512, 512)
            .position_centered()
            .build().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        let mut input_text: String = String::new();
        let mut input_width = 0;


        let input_surface = &self.font.render(self.initial_message.as_ref()).blended(self.text_colour).unwrap();
        input_width = input_surface.width();
        let mut input_texture = input_surface.as_texture(&texture_creator).unwrap();

        let mut input_text_changed = true;

        let mut event_pump = sdl_context.event_pump().unwrap();
        'gui_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'gui_loop;
                    }
                    Event::KeyDown { timestamp:_, window_id:_, keycode, scancode:_, keymod, repeat } => {
                        let mut name = keycode.unwrap().name();
                        let first = &name[..1];
                        if name == "Return" {
                            break 'gui_loop;
                        } else if name == "Backspace" {
                            input_text.pop();
                        } else if name == "Space" {
                            input_text.push(' ');
                        } else if name.len() == 1 && first >= "A" && first <= "Z" {
                            if keymod & sdl2::keyboard::Mod::CAPSMOD == sdl2::keyboard::Mod::CAPSMOD ||
                                keymod & sdl2::keyboard::Mod::LSHIFTMOD == sdl2::keyboard::Mod::LSHIFTMOD ||
                                keymod & sdl2::keyboard::Mod::RSHIFTMOD == sdl2::keyboard::Mod::RSHIFTMOD {

                            } else {
                                name = name.to_lowercase();
                            }
                            for c in name.chars() {
                                input_text.push(c)
                            }
                        } else if name.len() == 1 && first >= "0" && first <= "9" {
                            for c in name.chars() {
                                input_text.push(c);
                            }
                        }
                        input_text_changed=true;
                    }
                    _ => {}
                }
            }

            if input_text_changed {
                if input_text.len() > 0 {
                    let input_surface = &self.font.render(input_text.as_str()).blended(self.text_colour).unwrap();
                    input_width = input_surface.width();
                    input_texture = input_surface.as_texture(&texture_creator).unwrap();
                } else {
                    let mut faint = self.text_colour.clone();
                    faint.a = faint.a / 3;
                    let input_surface = &self.font.render(self.initial_message.as_ref()).blended(faint).unwrap();
                    input_width = input_surface.width();
                    input_texture = input_surface.as_texture(&texture_creator).unwrap();
                }
                input_text_changed=false;
            }

            canvas.set_draw_color(self.background_colour);
            canvas.clear();

            let input_rect = Rect::new(0, 400, input_width, HEIGHT);
            canvas.copy(&input_texture, None, input_rect).unwrap();
            canvas.present();
        }
        self
    }

    pub fn flash(self, sdl_context: &Sdl, video_subsystem: &sdl2::VideoSubsystem, seconds: u64) { //-> (Sdl, sdl2::VideoSubsystem) {
        let window = video_subsystem.window("window message", 512, 512)
            .position_centered()
            .borderless()
            .build().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        //let message = self.message.as_texture(&texture_creator).unwrap();


        canvas.set_draw_color(Color::RGB(0, 100, 100));
        canvas.clear();
        //let rect = Rect::new(100, 100, 200, 200);
        //canvas.copy(&message, None, rect);
        canvas.present();
        sleep(Duration::new(seconds, 0));
        //return (sdl_context,video_subsystem);
    }
}