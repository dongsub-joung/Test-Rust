use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(mut x: i128) -> (i128, i128){
    let mut line= 0i128;
    let mut max_num= 0i128;
    while x>max_num {
        line += 1;
        max_num += line;
    }

    let gap= max_num - x;
    let (mut top, mut under) = (0i128, 0i128);
    if line % 2 == 0{
        top= line - gap;
        under= gap + 1; 
    } else {
        top= gap + 1;
        under= line - gap;
    }

    (top, under)
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        
    }
}