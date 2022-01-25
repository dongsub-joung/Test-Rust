use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(a: i128, b: i128, c: i128) -> i128{
    let mut deb= -a;
    let mut cnt= 0;
    loop {
        deb += c-b;
        cnt += 1;

        if deb > 0{
            break;
        }
        if cnt > 2100000000{
            cnt = -1;
            break;
        }
    }

    cnt
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let (a,b,c)= (1000, 70, 170);
        assert_eq!(solution(a,b,c), 11);
    }
    #[test]
    fn test2(){
        let (a,b,c)= (3, 2, 1);
        assert_eq!(solution(a,b,c), -1);
    }
    #[test]
    fn test3(){
        let (a,b,c)= (2100000000, 9, 10);
        assert_eq!(solution(a,b,c), 2100000001);
    }

}