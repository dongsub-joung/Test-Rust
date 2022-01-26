use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(mut n: i128) -> i128{
    n -= 1;
    let mut i= 1;

    while n>0 {
        n -= i*6;
        i += 1;
    }

    i
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let input= 13;
        assert_eq!(solution(input), 3);
    }

}