use std::{io::{self, stdin}, num::IntErrorKind};

#[cfg(test)]
fn solution(phone_number: String) -> String{
    let size= phone_number.len()-4;
    let stars= &phone_number[0..size].to_string();
    let mut result= &phone_number[size..].to_string();
    
    let mut strs= String::new();
    for _ in 0..stars.len(){
        strs.push_str("*");
    }
    strs.push_str(result);
    
    strs.clone()
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let input= String::from("01033334444");
        let result= String::from("*******4444");
        assert_eq!(solution(input), result);
    }

    #[test]
    fn test2(){
        let input= String::from("027778888");
        let result= String::from("*****8888");
        assert_eq!(solution(input), result);
    }
}