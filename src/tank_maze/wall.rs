use crate::tank_maze::maze::Collide;

#[derive(Clone)]
#[derive(Copy)]
pub struct Wall {
    pub visible:bool,
    pub collide:Collide,
}

impl Wall {

    pub fn new(x:i32,y:i32,w:i32,h:i32) -> Wall {

        Wall{
            visible:true,
           collide:  Collide::new(x,y,w,h)
        }
    }
}