use std::fmt;
use crate::tank_maze::maze::{Collide, CELL_SIZE};
use crate::tank_maze::common;
use super::sdl2::rect::Point;
use crate::tank_maze::sound::{play, ENGINE, stop, volume};
use crate::tank_maze::projectile::Projectile;


#[derive(Clone)]
pub struct PlayerCar {
    pub collide: Collide,
    pub x: f64,
    pub y: f64,
    pub rotate: f64,
    pub width: u32,
    pub height: u32,
    velocity_x: f64,
    velocity_y: f64,
    velocity_rotate: f64,
    speed: f64,
    accelerate: bool,
    break_pedal: bool,
    pub bonus_time:i64,
    pub(crate) available_projectiles: usize,
    pub(crate) projectiles: Vec<Projectile>,
}


impl fmt::Display for PlayerCar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{} rotate {} velocity({},{})", self.x, self.y, self.rotate, self.velocity_x, self.velocity_y)
    }
}

impl PlayerCar {
    pub const MAX_PROJECTILES:usize = 4;

    pub fn new() -> PlayerCar {
        let width: u32 = 64;
        let height: u32 = 64;
        let p = PlayerCar {
            x: (CELL_SIZE as f64 / 2.0 - width as f64 / 4.0),
            y: (CELL_SIZE as f64 / 2.0 - height as f64 / 4.0),
            rotate: 0.0,
            width: width,
            height: height,
            collide: Collide::new(0, 0, width as i32, height as i32),
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_rotate: 0.0,
            speed: 0.0,
            accelerate: false,
            break_pedal: false,
            bonus_time:0,
            available_projectiles: 1,
            projectiles: vec![],
        };
        p
    }

    pub fn rollback(&mut self, previous: PlayerCar) {
        self.x = previous.x;
        self.y = previous.y;
        self.collide.rect.set_x(self.x as i32);
        self.collide.rect.set_y(self.y as i32);
    }

    pub fn update(&mut self, current_event: &common::Event) -> PlayerCar {
        let old: PlayerCar = (*self).clone();
        let rotate = 2.0;
        match current_event {
            common::Event::Left => self.velocity_rotate = -rotate,
            common::Event::Right => self.velocity_rotate = rotate,
            common::Event::Shift => {
                self.break_pedal = true;
                self.accelerate = false;
            }
            common::Event::Up => {
                self.accelerate = true;
                self.break_pedal = false;
                play(ENGINE);
            }
            common::Event::Space => {
                if self.available_projectiles > 0 {
                    let speed = 5.0;
                    let vel_x = speed * self.rotate.to_radians().cos();
                    let vel_y = speed * self.rotate.to_radians().sin();
                    let off_body: f64 = (self.height / speed as u32) as f64 / 2.4;
                    let p = Projectile::new(self.x + vel_x * off_body, self.y + vel_y * off_body, vel_x, vel_y);
                    self.projectiles.push(p);
                    self.available_projectiles = self.available_projectiles-1;
                }
            }
            common::Event::OffLeft => self.velocity_rotate = 0.0,
            common::Event::OffRight => self.velocity_rotate = 0.0,
            common::Event::OffUp => {
                self.velocity_x = 0.0;
                self.velocity_y = 0.0;
            }
            _ => {}
        }

        if self.break_pedal {
            self.break_pedal = false;
            self.speed = self.speed * 0.5;
            volume(ENGINE, self.speed as f32);
            if self.speed < 0.5 {
                self.speed = 0.0;
                stop(ENGINE);
            }
        }
        if self.accelerate {
            self.accelerate = false;
            if self.speed == 0.0 {
                self.speed = 1.0;
            }
            self.speed = self.speed * 1.25;
            volume(ENGINE, self.speed as f32);
        }
        if self.speed > 3.0 {
            self.speed = 3.0;
        }
        self.rotate = self.rotate + self.velocity_rotate;
        //if self.rotate != 0.0 || self.speed != 0.0 {
        self.new_direction();
        //}

        self.x = self.x + self.velocity_x;
        self.y = self.y + self.velocity_y;
        self.collide.rect.set_x(self.x as i32);
        self.collide.rect.set_y(self.y as i32);

        for p in self.projectiles.iter_mut() {
            p.update();
        }

        let mut i = self.projectiles.len() as i32 - 1;
        while i >= 0 {
            let p = self.projectiles.get(i as usize).unwrap();
            if p.finished {
                self.projectiles.remove(i as usize);
            }
            i = i - 1;
        }

        return old;
    }

    fn new_direction(&mut self) {
        self.velocity_x = self.speed * self.rotate.to_radians().cos();
        self.velocity_y = self.speed * self.rotate.to_radians().sin();
    }
}

pub fn get_rotated(player: &PlayerCar) -> [Point; 13] {
    let centre_x = (player.x + 0.0) as i32;
    let centry_y = (player.y + 0.0) as i32;
    let points = [
        // sort out collision detection
        gp(40.0, player.rotate - 34.0).offset(centre_x, centry_y),
        gp(40.0, player.rotate - 22.0).offset(centre_x, centry_y),
        gp(40.0, player.rotate - 11.0).offset(centre_x, centry_y),
        gp(40.0, player.rotate).offset(centre_x, centry_y),
        gp(40.0, player.rotate + 11.0).offset(centre_x, centry_y),
        gp(40.0, player.rotate + 22.0).offset(centre_x, centry_y),
        gp(40.0, player.rotate + 34.0).offset(centre_x, centry_y),
        gp(38.0, player.rotate - 45.0).offset(centre_x, centry_y),
        gp(38.0, player.rotate + 45.0).offset(centre_x, centry_y),
        gp(38.0, player.rotate + 60.0).offset(centre_x, centry_y),
        gp(38.0, player.rotate - 60.0).offset(centre_x, centry_y),
        gp(38.0, player.rotate + 90.0).offset(centre_x, centry_y),
        gp(38.0, player.rotate - 90.0).offset(centre_x, centry_y),
    ];


    return points;
}

fn gp(r: f64, a: f64) -> Point {
    return Point::new((a.to_radians().cos() * r) as i32, (a.to_radians().sin() * r) as i32);
}