use data_structures_rs::quadtree;
use std::rc::Rc;
use nannou::prelude::*;

const QT_SIZE: f32 = 400.0;

fn main() {
    nannou::sketch(view).run();
}

//NANNOU
fn view(app: &App, frame: Frame) {
    let mut qt = quadtree::Quadtree::new(QT_SIZE * -0.5, QT_SIZE * 0.5, QT_SIZE, QT_SIZE);
    let rect = Rectangle::new(1.0, 1.0, 2.0, 2.0);
    match qt.insert(Rc::new(rect)) {
        Ok(_) => {
            let mut vec: Vec<Rc<dyn quadtree::Sized>> = vec![];
            let rect = Rectangle::new(190.0, 190.0, 2.0, 2.0);
            let _ = qt.insert(Rc::new(rect));
            let rect: Rc<dyn quadtree::Sized> = Rc::new(Rectangle::new(QT_SIZE * -0.5 + 1.0, QT_SIZE * 0.5 - 1.0, QT_SIZE * 0.9, QT_SIZE * 0.9));
            let r = qt.get_rect(Rc::clone(&rect), &mut vec);
            println!("{:?} - {:?}", r, vec);

            let draw = app.draw();
            draw.background().rgb(0.0, 0.0, 0.0);
            draw.to_frame(app, &frame).unwrap();
        },
        Err(e) => eprintln!("{}", e),
    }
}

// QUADTREE
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

impl quadtree::Sized for Rectangle {
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
