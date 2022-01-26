use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(a: i128, b: i128, v: i128) -> i128{
    let mut day= 0i128;
    day= ((v-a) / (a-b) + 1); 

    day
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(solution(2, 1, 5), 4);
    }

    #[test]
    fn test2(){
        assert_eq!(solution(100, 99, 1000000000), 999999901
    );
    }
}