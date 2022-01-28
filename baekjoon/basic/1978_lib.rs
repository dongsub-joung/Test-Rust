use std::{io::{self, stdin}};

// f64, Rounding
#[cfg(test)]
fn solution(a: usize, b: &str) -> u128{
    if a > 100 {
        panic!(" n size is more less 100.")
    }

    let mut cnt= 0_u128; 
    let mut v: Vec<usize>= Vec::new();
    let list= b.split_whitespace();
    
    for i in list{
        let value= i.parse::<usize>().unwrap();
        if value > 1000{
            panic!("element size is more less 1000")
        }
        v.push(value);
    }
    
    if v.len() > a {
        panic!("Out Range");
    } else {
        for i in v.iter(){
            for j in (2..*i){
                if i % j == 0 {
                    continue;
                } else {
                    if j == i-1{
                        cnt +=1;
                    }
                }
            }
        }
    }

    cnt
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(solution(4, "1 3 5 7"), 3);
    }
}