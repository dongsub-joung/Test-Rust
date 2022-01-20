#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main()
{
    let rect1= Rectangle{ length: 50, width: 30 };
    // println!("rect1 is {:?}", rect1);        한줄에서 출력
    println!("rect1 is {:#?}", rect1);   //    여러줄로 출력
}
