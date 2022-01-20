fn ex_if()
{
    let x= 5;
    if x != 0 
    {
        println!("X is not zero.");
    }
}    

fn ex_if_return()
{
    let condition= true;
    let number= if condition 
    {
        5
    } 
    else 
    {
        6
    };
    println!("number is {}", number);
}

fn ex_while()
{
    let mut z= 3;
    while z != 0 
    {
        println!("{}!", z);
        z-= 1;
    }
    println!("LIFTOFF!!!");
}

fn ex_for()
{
   let a = [10, 20, 30, 40, 50];

   for element in a.iter()
   {
       println!("the value is {}", element);
   }
}

fn ex_for_range()
{
    // range (1~4) reverse => number
    // 파이썬과 동일하게 n-1까지 실행 
    for number in (1..4).rev() 
    {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main()
{
    // ex_if();
    // ex_if_return();
    // ex_while();
    // ex_for();
    ex_for_range();
}
