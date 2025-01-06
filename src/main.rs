trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];

    for shape in shapes {
        shape.draw();
    }
}