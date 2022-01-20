fn main() 
{
    let x= 5;
    println!("The value of x is: {}", x);

    const Y: i16= 6;
    println!("The value of x is: {}", Y);

    let z= add_five(2);
    println!("The value of z is: {}", z);
}

fn add_five(m: i32) -> i32
{
    m+5
}
