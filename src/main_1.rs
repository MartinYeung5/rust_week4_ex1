use crate::Shape::Rectangle;
use crate::Shape::Circle;
use crate::Shape::RightAngleTriangle;

#[derive(Clone)]
enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    RightAngleTriangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => 
                width * height,
            Shape::Circle { radius } => 
                std::f64::consts::PI * radius.powi(2),
            Shape::RightAngleTriangle { base, height } => 
                0.5 * base * height,
        }
    }
}

fn main() {
    let mut vec: Vec<Shape> = Vec::new();
    println!("Add to vec");
    vec.push(Rectangle { width: 2.0, height: 3.0 });
    println!("vec record is {}", vec.len());
    println!("Add to vec");
    vec.push(Circle { radius: 3.0 });
    println!("vec record is {}", vec.len());
    println!("Add to vec");
    vec.push(RightAngleTriangle { base: 2.0, height: 3.0 });
    println!("vec record is {}", vec.len());
}