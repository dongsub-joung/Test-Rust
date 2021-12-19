#[derive(Debug)]
struct Rectangle {
    lenght: u32,
    width:  u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.lenght * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.lenght && self.width > other.width
    }
}

fn main(){
    let rect1= Ractangle { lenght: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// associated functions
impl Rectangle{
    fn sequare(size: u32) -> Rectangle {
        Rectangle { lenght: size, width: size }
    }
}
fn main(){
    println!(
        "The area of the rectangle is {} square pixels.",
        Rectangle::sequare(50, 30)
    );
}


// automatic referencing and dereferencing
