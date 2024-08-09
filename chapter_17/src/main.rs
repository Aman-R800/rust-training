use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        Box::new(rectangle),
    ];

    for shape in shapes {
        println!("Shape area: {}", shape.area());
    }
}
