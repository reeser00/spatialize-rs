# Spatialize - Spatial Data Structures

`spatialize` provides data-structures for efficient spatial partitioning.
[crates.io](https://crates.io/crates/spatialize)

## Documentation
[Check out the doc at docs.rs](https://docs.rs/spatialize)

## Includes
 - Quadtree

## Quadtree Example

 1. Implement the `Sized` trait for the object you want to store in the `Quadtree`.
```rust
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
```

2. Create a `Quadtree` with the your given boundaries and `insert()` your object that implements the `Sized` trait.
3. Get all objects within a given `rect_view` by passing it into `get_rect()` with a Vector.

```rust
let position_x: f32 = -100.0;
let position_y: f32 = 100.0;
let width: f32 = 200.0;
let height: f32 = 200.0;
let mut qt = Quadtree::new(position_x, position_y, width, height);

let sized_object: Rc<dyn Sized> = Rc::new(Rectangle::new(0.0, 0.0, 5.0, 5.0));
match qt.insert(Rc::clone(&sized_object)) {
      Ok(_) => {

          let rect_view: Rc<dyn Sized> = Rc::new(Rectangle::new(-2, 2, 10.0, 10.0));
          let mut result_vec: Vec<Rc<dyn Sized>> = vec![];
          match qt.get_rect(rect_view, &mut result_vec) {
              Ok(_) => assert_eq!(1, result_vec.len()),
              Err(e) => eprintln!("{}", e),
          }

      },
      Err(e) => eprintln!("{}", e),
}
```
