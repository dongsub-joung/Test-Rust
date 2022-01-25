use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(arr: Box<[f64]>) -> f64{
    let mut sum= 0.0;
    let number= arr.len() as f64;
    for i in arr.iter() {
        sum += i;
    }

    sum / number 
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let data= Box::new([1.0, 2.0, 3.0, 4.0]);
        let result= 2.5;
        assert_eq!(solution(data), result);
    }

    #[test]
    fn test2(){
        let data= Box::new([5.0 ,5.0]);
        let result= 5.0;
        assert_eq!(solution(data), result);
    }
}