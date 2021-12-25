fn main()
{
    // 매직 넘버가 되어버림
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 
{
    // 어떤 값인지 모름
    dimensions.0 * dimensions.1
}

// => add_meaning.rs
