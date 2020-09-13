use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::tank_maze::extra_prizes::{Prize, PrizeType};
use crate::tank_maze::wall::Wall;

use super::sdl2::rect::{Point, Rect};

#[derive(Clone)]
#[derive(Copy)]
pub struct Collide {
    pub rect: Rect,
}

pub struct Maze {
    pub maze_object: Vec<Wall>,
    pub end_x_y: (i32, i32),
    pub squares: u32,
    pub bonus_items: Vec<Prize>,
}

impl Collide {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Collide {
        let r = Rect::new(x, y, w as u32, h as u32);

        Collide {
            rect: r,
        }
    }
}

const NORTH: usize = 0;
const SOUTH: usize = 2;
const EAST: usize = 1;
const WEST: usize = 3;

#[derive(Clone)]
struct Cell {
    pub neighbours: Vec<(usize, usize)>,
    pub walls: Vec<Wall>,
    pub visited: bool,
    pub end: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            neighbours: vec![],
            walls: vec![Wall::new(0, 0, 0, 0); 4],
            visited: false,
            end: false,
        }
    }
}

pub const CELL_SIZE: usize = 128;

impl Maze {
    pub fn new(level: u32) -> Maze {
        let mut grid: Vec<Wall> = vec![];

        let width = 8 + level as usize - 1;
        let height = 8 + level as usize - 1;
        let mut grid_raw = vec![Cell::new(); width * height];
        let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

        let mut cell_grid = grid_base.as_mut_slice();

        let wall_width = 3;

        for x in 0..width {
            for y in 0..height {
                cell_grid[y][x].walls[NORTH] = Wall::new((x * CELL_SIZE) as i32, (y * CELL_SIZE) as i32, CELL_SIZE as i32, wall_width);
                cell_grid[y][x].walls[EAST] = Wall::new((x * CELL_SIZE) as i32 + CELL_SIZE as i32, (y * CELL_SIZE) as i32, wall_width, CELL_SIZE as i32);
                cell_grid[y][x].walls[SOUTH] = Wall::new((x * CELL_SIZE) as i32, (y * CELL_SIZE) as i32 + CELL_SIZE as i32, CELL_SIZE as i32, wall_width);
                cell_grid[y][x].walls[WEST] = Wall::new((x * CELL_SIZE) as i32, (y * CELL_SIZE) as i32, wall_width, CELL_SIZE as i32);
                let north = y as i32 - 1;
                let south = y as i32 + 1;
                let east = x as i32 + 1;
                let west = x as i32 - 1;
                if north >= 0 && north < height as i32 {
                    cell_grid[y][x].neighbours.push((north as usize, x));
                }
                if south >= 0 && south < height as i32 {
                    cell_grid[y][x].neighbours.push((south as usize, x));
                }
                if east >= 0 && east < width as i32 {
                    cell_grid[y][x].neighbours.push((y, east as usize));
                }
                if west >= 0 && west < width as i32 {
                    cell_grid[y][x].neighbours.push((y, west as usize));
                }
            }
        }

        cell_grid[0][0].visited = true;
        let mut stack = vec![(0, 0)];

        let (squares, end_x_y) = Maze::find_a_path_through_grid(&mut cell_grid, &mut stack, height, width);

        for x in 0..width {
            for y in 0..height {
                if x >= 1 {
                    if cell_grid[y][x].walls[EAST].visible {
                        grid.push(cell_grid[y][x].walls[EAST]);
                    }
                } else {
                    if cell_grid[y][x].walls[EAST].visible {
                        grid.push(cell_grid[y][x].walls[EAST]);
                    }
                    if cell_grid[y][x].walls[WEST].visible {
                        grid.push(cell_grid[y][x].walls[WEST]);
                    }
                }
                if y >= 1 {
                    if cell_grid[y][x].walls[SOUTH].visible {
                        grid.push(cell_grid[y][x].walls[SOUTH]);
                    }
                } else {
                    if cell_grid[y][x].walls[NORTH].visible {
                        grid.push(cell_grid[y][x].walls[NORTH]);
                    }
                    if cell_grid[y][x].walls[SOUTH].visible {
                        grid.push(cell_grid[y][x].walls[SOUTH]);
                    }
                }
            }
        }

        let mut extras: Vec<Prize> = vec![];

        let mut rng = rand::thread_rng();
        for xx in 1..width - 1 {
            for yy in 1..height - 1 {
                let random = rng.gen_range(1, 100);
                if random > 95 {
                    let e = if yy % 2 == 0 {
                        Prize::new((xx * CELL_SIZE) as f64, (yy * CELL_SIZE) as f64, PrizeType::ExtraProjectiles)
                    } else {
                        Prize::new((xx * CELL_SIZE) as f64, (yy * CELL_SIZE) as f64, PrizeType::ExtraTime)
                    };
                    extras.push(e);
                }
            }
        }

        Maze {
            maze_object: grid,
            end_x_y: end_x_y,
            squares: squares,
            bonus_items: extras,
        }
    }

    fn find_a_path_through_grid(cell_grid: &mut &mut [&mut [Cell]], stack: &mut Vec<(usize, usize)>, height: usize, width: usize) -> (u32, (i32, i32)) {
        let mut squares: u32 = 0;
        let mut end_x_y: (i32, i32) = (0, 0);

        while stack.len() > 0 {
            let current = stack.pop().unwrap();
            squares = squares + 1;

            let mut use_neighbour = (999, 999);
            cell_grid[current.0][current.1].neighbours.shuffle(&mut thread_rng());

            for neighbour in cell_grid[current.0][current.1].neighbours.iter() {
                let check = &cell_grid[neighbour.0][neighbour.1];
                if check.visited == false {
                    use_neighbour = (neighbour.0, neighbour.1);
                    break;
                }
            }
            if use_neighbour.0 < 999 && use_neighbour.1 < 999 {
                stack.push((current.0, current.1));

                cell_grid[use_neighbour.0][use_neighbour.1].visited = true;
                // remove wall
                if use_neighbour.0 > current.0 { // neighbour below
                    cell_grid[current.0][current.1].walls[SOUTH].visible = false;
                    cell_grid[use_neighbour.0][use_neighbour.1].walls[NORTH].visible = false;
                }
                if use_neighbour.0 < current.0 { // neighbour above
                    cell_grid[current.0][current.1].walls[NORTH].visible = false;
                    cell_grid[use_neighbour.0][use_neighbour.1].walls[SOUTH].visible = false;
                }
                if use_neighbour.1 > current.1 { // neighbour right
                    cell_grid[current.0][current.1].walls[EAST].visible = false;
                    cell_grid[use_neighbour.0][use_neighbour.1].walls[WEST].visible = false;
                }
                if use_neighbour.1 < current.1 { // neighbour left
                    cell_grid[current.0][current.1].walls[WEST].visible = false;
                    cell_grid[use_neighbour.0][use_neighbour.1].walls[EAST].visible = false;
                }
                stack.push((use_neighbour.0, use_neighbour.1));
            } else {
                cell_grid[current.0][current.1].end = true;
                ((current.1 * CELL_SIZE + CELL_SIZE / 2) as i32, (current.0 * CELL_SIZE + CELL_SIZE / 2) as i32);
                end_x_y = (((height - 1) * CELL_SIZE + CELL_SIZE / 2) as i32, ((width - 1) * CELL_SIZE + CELL_SIZE / 2) as i32);
            }
        }
        (squares, end_x_y)
    }

    pub fn collision(&self, points: &[Point]) -> bool {
        for w in self.maze_object.iter() {
            for p in points {
                if w.visible && w.collide.rect.contains_point(*p) == true {
                    return true;
                }
            }
        }
        return false;
    }
    pub fn collision_remove_wall(&mut self, points: &[Point]) -> bool {
        for w in self.maze_object.iter_mut() {
            for p in points {
                if w.visible && w.collide.rect.contains_point(*p) == true {
                    w.visible = false;
                    return true;
                }
            }
        }
        return false;
    }
}