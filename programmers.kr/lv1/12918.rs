use std::{io::{self, stdin}, num::IntErrorKind};

#[cfg(test)]
fn solution(str: String) -> bool{
    let size= str.len();
    if size == 4 || size == 6{
        if numbers_only(str) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn numbers_only(str: String) -> bool{
    for i in str.split(""){
        match i.parse::<usize>() {
            // match err handler
            Err=> { return false;}
            _ => { continue; }
        }
    }
    return true;
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let input= String::from("a234");
        assert_eq!(solution(input), false);
    }

    #[test]
    fn test2(){
        let input= String::from("1234");
        assert_eq!(solution(input), true);
    }
}