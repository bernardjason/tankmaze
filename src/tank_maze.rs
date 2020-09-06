extern crate pretty_env_logger;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use self::sdl2::pixels::Color;
use crate::tank_maze::common::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::tank_maze::sound::load_sound;
use crate::tank_maze::message_area::MessageArea;
use std::time::SystemTime;
use rand::{random, Rng};


mod common;
mod start_screen;
mod main_screen;
mod game_end_screen;
mod player_car;
mod maze;
mod wall;
mod sound;
mod projectile;
mod extra_prizes;
mod message_area;

pub fn tank_maze() -> Result<(), String> {
    let mut sdl_context = sdl2::init()?;
    let mut video_subsystem = sdl_context.video()?;
    let mut frame_control = FPSManager::new();
    let mut current_screen = 0;

    frame_control.set_framerate(60)?;


    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = "artifacts/OpenSans-Light.ttf";
    let mut font = ttf_context.load_font(font_path, 32)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut message_font = ttf_context.load_font(font_path, 32)?;
    message_font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let mut input_font = ttf_context.load_font(font_path, 32)?;
    let mut message = MessageArea::new(&mut input_font,
                                       "Enter name please".parse().unwrap(), "Hello TEST".parse().unwrap(),
                                       Color::YELLOW, Color::WHITE, "Baner text".parse().unwrap());
    //message = message.display(&sdl_context, &video_subsystem);

    let window = video_subsystem.window("tank maze", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    load_sound(&sdl_context);

    let mut start_screen = start_screen::StartScreen::new(&texture_creator, &font);
    let mut main_screen = main_screen::MainScreen::new(&texture_creator, &font);
    let mut game_over_screen = game_end_screen::GameEndScreen::new(&texture_creator, &font);

    let mut message_area_active = false;

    let mut event_pump = sdl_context.event_pump()?;
    'gui_loop: loop {
        sound::pause_any_finished_sounds();

        let mut current_event = common::Event::None;

        if message_area_active {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'gui_loop;
                    }
                    Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => {
                        message = message.key_down_handle(keycode.unwrap(), keymod);
                        message_area_active = !message.input_text_complete;
                    }
                    _ => {}
                }
            }
        } else {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'gui_loop;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        current_event = common::Event::Left
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        current_event = common::Event::Right
                    }
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        current_event = common::Event::Up
                    }
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        current_event = common::Event::Space
                    }
                    Event::KeyDown { keycode: Some(Keycode::LShift), .. } => {
                        current_event = common::Event::Shift
                    }
                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                        current_event = common::Event::off_Left
                    }
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                        current_event = common::Event::off_Right
                    }
                    Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                        current_event = common::Event::off_Up
                    }
                    Event::KeyUp { keycode: Some(Keycode::LShift), .. } => {
                        current_event = common::Event::off_Shift
                    }
                    Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                        current_event = common::Event::off_Space
                    }
                    Event::KeyDown { keycode: Some(Keycode::Y), .. } => {
                        main_screen.took = 99999;
                        main_screen.level_done = true;
                        main_screen.lives = main_screen.lives  -1;
                    }

                    _ => {}
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        match current_screen {
           0 => {
               start_screen.update();
               start_screen = start_screen.draw_on_canvas(&mut canvas);
               if start_screen.bernie_x > (SCREEN_WIDTH / 3) as i32 || current_event == common::Event::Space {
                   current_screen = 1;
                   message_area_active = false;
                   main_screen = main_screen.new_game();
               }
           }
           1 => {
               if main_screen.update(&current_event) {
                   main_screen = main_screen.draw_on_canvas(&mut canvas, &message_font, &texture_creator);
                   message_area_active = false;
               } else {
                   current_screen = 2;
                   game_over_screen.change_to_screen(main_screen.bonus_points);
                   message.input_text.clear();
                   message.input_text_complete = false;
                   message_area_active = true;
               }
           }
            _ => {
                game_over_screen.update(message.input_text.clone(),message.input_text_complete);
                game_over_screen = game_over_screen.draw_on_canvas(&mut canvas,&message_font,&texture_creator);
                if game_over_screen.put_on_table == false {
                    message_area_active=false;
                }
                if message_area_active {
                     message.on_canvas(&mut canvas,0,200);
                }
                if message_area_active == false && current_event == common::Event::Space {
                    start_screen.bernie_x = 0;
                    current_screen = 0;
                    message_area_active = false;
                }

            }
        }

        canvas.present();

        let rate = frame_control.delay();
    }
    Ok(())
}

