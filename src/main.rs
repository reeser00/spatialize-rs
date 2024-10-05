use data_structures_rs::quadtree::{Quadtree, Sized};
use std::rc::Rc;

fn main() {}

#[derive(Debug)]
struct Rectangle {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(position_x: f32, position_y: f32, width: f32, height: f32) -> Self {
        Self {
            position_x,
            position_y,
            width,
            height,
        }
    }
}

impl Sized for Rectangle {
    fn north_edge(&self) -> f32 {
        self.position_y
    }
    fn east_edge(&self) -> f32 {
        self.position_x + self.width
    }
    fn south_edge(&self) -> f32 {
        self.position_y - self.height
    }
    fn west_edge(&self) -> f32 {
        self.position_x
    }
}
