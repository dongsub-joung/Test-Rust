// https://doc.rust-lang.org/rust-by-example/fn/methods.html
struct Point {
    x: f64,
    y: f64,
}

impl Poin {
    fn origin() -> Point{
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point{x: x, y: y}
    }
}

struct Rectangle{
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64{
        let Point{ x: x1, y: y2 }= self.p1;
        let Point{ x: x1, y: y2 }= self.p2;
        
        ((x1-x2)*(y1-y2)).abs()
    }
    
    fn perimeter(&self) -> f64{
        let Point{ x: x1, y: y2 }= self.p1;
        let Point{ x: x1, y: y2 }= self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64){
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destroy(slef){
        let Pair(first, second)= self;
        println!("Destroying Pair({}, {})", first, second);
        // / `first` and `second` go out of scope and get freed
    }
}

fn main(){
    let rectangle= Rectangle{
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    
    let mut square= Rectangle{
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    }
    
    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);
}