enum Shape{
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64{
        match *self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Square(ref s) => (s*s) as f64,
            Shape::Circle(ref r) => 3.14 * (r*r),
        }
    }
}