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
        fn square(size: u32) -> Rectangle 
        {
            Rectangle { length: size, width: size }
        }
    }

    let sq = Rectangle::square(3);
    println!("{:?}", sq);
}
