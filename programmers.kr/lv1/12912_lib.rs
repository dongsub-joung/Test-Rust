use std::io::{self, stdin};

#[cfg(test)]
fn solution(mut a: usize, mut b: usize) -> usize{
    let mut answer= 0usize;
    
    if a>b {
        let temp= a;
        a= b;
        b= temp;
    }

    if a != b{
        for i in a..b+1{
            answer+= i;
        }
    } else {
        answer= a;    
    }

    answer
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let result=solution(3, 5);
        assert_eq!(result, 12);
    }

    #[test]
    fn test2(){
        let result= solution(3,3);
        assert_eq!(result, 3);
    }
    #[test]
    fn test3(){
        let result= solution(5,3);
        assert_eq!(result, 12);
    }
}