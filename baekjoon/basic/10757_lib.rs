use std::{io::{self, stdin}};

// f64, Rounding
#[cfg(test)]
fn solution(a: u128, b: u128) -> u128{
    a + b
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(solution(9223372036854775807, 9223372036854775808), 18446744073709551615);
    }
}