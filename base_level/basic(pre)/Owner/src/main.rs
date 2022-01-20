fn gives_ownership() -> String 
{
    let str= String::from("hello");
    str
}

fn takes_and_gives_back(str :String) -> String
{
    str
}

fn main()
{
    let s1= gives_ownership();
    let s3= takes_and_gives_back(s1);
    println!("s3 is {}", s3);
}
