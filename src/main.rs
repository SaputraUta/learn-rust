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

fn draw_dynamic(shape: &dyn Draw) {
    shape.draw();
}

fn main() {
    let circle = Circle;
    let square = Square;
    draw_dynamic(&circle);
    draw_dynamic(&square);
}