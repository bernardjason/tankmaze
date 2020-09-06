use crate::tank_maze::maze::Collide;
use std::ops::Deref;
use std::borrow::Borrow;
use super::sdl2::rect::Point;


#[derive(Clone)]
pub struct Projectile {
    pub collide: Collide,
    pub point:[Point;3],
    pub x: f64,
    pub y: f64,
    velocity_x: f64,
    velocity_y: f64,
    clicks:i32,
    pub finished:bool,
}
pub const  WIDTH:i32=4;
pub const  HEIGHT:i32=4;
impl Projectile {
    pub fn new(start_x:f64,start_y:f64,direction_x:f64,direction_y:f64) -> Projectile {

        Projectile {
            collide:Collide::new(start_x as i32,start_y as i32,WIDTH,HEIGHT),
            point:[Point::new(start_x as i32, start_y as i32);3],
            x:start_x,
            y:start_y,
            velocity_y:direction_y,
            velocity_x:direction_x,
            clicks:0,
            finished:false,
        }
    }
    pub fn update(&mut self) -> &mut Projectile {
        self.x=self.x+self.velocity_x;
        self.y=self.y+self.velocity_y;
        self.point = [
            Point::new(self.x as i32, self.y as i32),
            Point::new((self.x + self.velocity_x*1.5) as i32, (self.y + self.velocity_y*1.5) as i32),
            Point::new((self.x + self.velocity_x*3.0) as i32, (self.y + self.velocity_y*3.0) as i32),
        ];
        self.clicks=self.clicks+1;
        self.collide.rect.set_x(self.x as i32);
        self.collide.rect.set_y(self.y as i32);
        if self.clicks > 100 {
            self.finished = true;
        }

        return self;
    }
}