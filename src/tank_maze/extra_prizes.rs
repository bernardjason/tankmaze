use crate::tank_maze::maze::{Collide, CELL_SIZE};

pub enum PrizeType {
    ExtraProjectiles,
    ExtraTime,
}
pub struct Prize {
    pub type_of_prize: PrizeType,
    pub collide: Collide,
    pub x: f64,
    pub y: f64,
    pub still_valid:bool,
}

impl Prize {
    pub fn new(x:f64, y:f64, type_of_prize: PrizeType) -> Prize {
        Prize{
            type_of_prize,
            collide: Collide::new(x as i32, y as i32, CELL_SIZE as i32 /2, CELL_SIZE as i32 /2),
            x,
            y,
            still_valid: true
        }
    }
}