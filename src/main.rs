trait Side {
    fn side(&self) -> f64;
}

struct Rectangle {
    height: f64,
    width: f64,
}

impl Side for Rectangle {
    fn side(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}


fn main() {
    let mut vec2: Vec<&dyn Side> = Vec::new();
    let shapes: Vec<Box<dyn Side>> = vec![
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
    ];

    for side in &shapes {
        println!("{}", side.side());
    }
}