use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Quadtree {
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    divided: bool,
    northeast_quad: Option<Rc<RefCell<Self>>>,
    northwest_quad: Option<Rc<RefCell<Self>>>,
    southeast_quad: Option<Rc<RefCell<Self>>>,
    southwest_quad: Option<Rc<RefCell<Self>>>,
    contents: Vec<Rc<dyn Sized>>,
}

pub trait Sized: Debug {
    fn north_edge(&self) -> f32;
    fn east_edge(&self) -> f32;
    fn south_edge(&self) -> f32;
    fn west_edge(&self) -> f32;
}

impl Quadtree {
    pub fn new(position_x: f32, position_y: f32, width: f32, height: f32) -> Self {
        Self {
            position_x,
            position_y,
            width,
            height,
            divided: false,
            northeast_quad: None,
            northwest_quad: None,
            southeast_quad: None,
            southwest_quad: None,
            contents: vec![],
        }
    }

    fn subdivide(&mut self) {
        if !self.divided {
            self.northeast_quad = Some(Rc::new(RefCell::new(Quadtree::new(self.position_x + self.width / 2.0, self.position_y, self.width / 2.0, self.height / 2.0))));
            self.northwest_quad = Some(Rc::new(RefCell::new(Quadtree::new(self.position_x, self.position_y, self.width / 2.0, self.height / 2.0))));
            self.southeast_quad = Some(Rc::new(RefCell::new(Quadtree::new(self.position_x + self.width / 2.0, self.position_y - self.height / 2.0, self.width / 2.0, self.height / 2.0))));
            self.southwest_quad = Some(Rc::new(RefCell::new(Quadtree::new(self.position_x, self.position_y - self.height / 2.0, self.width / 2.0, self.height / 2.0))));
            self.divided = true;
        }
    }

    pub fn insert(&mut self, sized_object: Rc<dyn Sized>) -> Result<(), String> {
        if sized_object.north_edge() <= self.position_y &&
            sized_object.east_edge() <= self.position_x + self.width &&
            sized_object.south_edge() >= self.position_y - self.height &&
            sized_object.west_edge() >= self.position_x {

                //Object fits in Quadtree
                if !self.divided {
                    self.subdivide();
                }
                if let Some(rc_ref) = &self.northeast_quad {
                    if let Ok(_) = rc_ref.borrow_mut().insert(Rc::clone(&sized_object)) {
                        return Ok(());
                    }
                }
                if let Some(rc_ref) = &self.northwest_quad {
                    if let Ok(_) = rc_ref.borrow_mut().insert(Rc::clone(&sized_object)) {
                        return Ok(());
                    }
                }
                if let Some(rc_ref) = &self.southeast_quad {
                    if let Ok(_) = rc_ref.borrow_mut().insert(Rc::clone(&sized_object)) {
                        return Ok(());
                    }
                }
                if let Some(rc_ref) = &self.southwest_quad {
                    if let Ok(_) = rc_ref.borrow_mut().insert(Rc::clone(&sized_object)) {
                        return Ok(());
                    }
                }

                //Object doesn't fit in any divisions
                self.contents.push(sized_object);
                Ok(())
        } else {
            Err(String::from("Object doesn't fit within the Quadtree bounds."))
        }
    }

    pub fn get_rect(&self, rect: Rc<dyn Sized>, vec: &mut Vec<Rc<dyn Sized>>) -> Result<(), String> {
        if !(rect.north_edge() < self.position_y - self.height ||
            rect.east_edge() < self.position_x ||
            rect.south_edge() > self.position_y ||
            rect.west_edge() > self.position_x + self.width) {
                if self.divided {
                    if let Some(rc_ref) = &self.northeast_quad {
                        let _ = rc_ref.borrow().get_rect(Rc::clone(&rect), vec);
                    }
                    if let Some(rc_ref) = &self.northwest_quad {
                        let _ = rc_ref.borrow().get_rect(Rc::clone(&rect), vec);
                    }
                    if let Some(rc_ref) = &self.southeast_quad {
                        let _ = rc_ref.borrow().get_rect(Rc::clone(&rect), vec);
                    }
                    if let Some(rc_ref) = &self.southwest_quad {
                        let _ = rc_ref.borrow().get_rect(Rc::clone(&rect), vec);
                    }
                }
                for rc in self.contents.iter() {
                    vec.push(Rc::clone(&rc));
                }
                Ok(())
        } else {
            Err(String::from("Rectangle doesn't overlap the Quadtree bounds."))
        }
    }
}
