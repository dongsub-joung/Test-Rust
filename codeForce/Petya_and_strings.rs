use std::io::{self, stdin};

fn inputing() -> String {
    let mut word= String::new();
    stdin().read_line(&mut word).unwrap();
    word.trim().to_string()
}

fn main(){
    let word= inputing();
    let word2= inputing();

    let size= word.len();
    let size2= word2.len();
    
    if size <= 100 && size2 <= 100{
        let a= word.to_lowercase();
        let b= word2.to_lowercase();
        
        let array1= a.as_bytes();
        let array2= b.as_bytes();

        let mut sum1= 0;
        let mut sum2= 0;
        
        for i in array1{
            sum1 += i;
        }

        for i in array2{
            sum2 += i;
        }

        if sum1 == sum2 {
            println!("{}", 0);
        } else if sum1> sum2{
            println!("{}", 1);
        } else {
            println!("{}", -1);
        }
    } else {
        panic!("err")
    }
}