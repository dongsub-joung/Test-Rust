
struct Rectangle 
{
    // type만 선언
    length: u32,
    width: u32,
}

fn main() 
{
    // key: value 초기화 (Rectangle 초기화)
    let rect1= Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32
{
    rectangle.length * rectangle.width
}

// 매직 넘버가 사라지고 의미가 명화해짐.
