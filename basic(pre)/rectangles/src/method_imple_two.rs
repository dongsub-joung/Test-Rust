
#![allow(unused)]
fn main() 
{
    #[derive(Debug)]
    struct Rectangle 
    {
        length: u32,
        width: u32,
    }

    impl Rectangle 
    {
        fn area(&self) -> u32 
        {
            self.length * self.width
        }

        fn can_hold(&self, other: &Rectangle) -> bool
        {
            self.length > other.length && self.width > other.width
        }
    }

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
