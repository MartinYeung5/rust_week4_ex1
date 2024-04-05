trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    height: f64,
    width: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

struct RightAngleTriangle {
    base: f64,
    height: f64,
}

impl Shape for RightAngleTriangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn main() {
    let mut vec2: Vec<&dyn Shape> = Vec::new();
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Circle { radius: 4.0 }),
        Box::new(RightAngleTriangle { base: 2.0, height: 3.0  })
    ];

    for shape in &shapes {
        println!("{}", shape.area());
    }
}