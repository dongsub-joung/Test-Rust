use std::{io::{self, stdin}};

// f64, Rounding
#[cfg(test)]
fn solution(mut n: i128) -> i128{
    let mut result= 0i128;
    let divid_five= n % 5;
    let divid_three= n % 3;
    let divid_sum=  n % 8 ;

    if divid_three != 0 && divid_five != 0 && divid_sum != 0{
        result= -1;
    } else {
        result += n / 5;
        n= divid_five;
        result += n / 3;
    }
    result
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(solution(18), 4);
    }

    #[test]
    fn test2(){
        assert_eq!(solution(4), -1);
    }

    #[test]
    fn test3(){
        assert_eq!(solution(6), 2);
    }

    #[test]
    fn test4(){
        assert_eq!(solution(9), 3);
    }

}