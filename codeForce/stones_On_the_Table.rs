use std::io;
fn inputing_number() -> u8{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    
    buf.trim().parse::<u8>().unwrap()
}
fn inputing_String() -> String{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    
    buf.trim().to_string()
}

fn main(){
    let mut save= 0u8;
    let mut result= 0u8;
    
    let n= inputing_number();
    let s= inputing_String();
    
    for element in s.as_bytes().iter(){
        if *element == save{
            result = result+1;
        }
        save= *element;
    }
    
    println!("{}", result);
}