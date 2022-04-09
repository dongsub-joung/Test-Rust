use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(id_list: Vec<&str>, report: Vec<&str>, k: usize) -> Vec<usize>{
    let size= id_list.len();
    let mut result= vec![0; size];
    let check: Vec<(usize, usize)>= Vec::new();

    let mut p= vec![0; size];
    let mut point: Vec<i32> = Vec::new();
    for _ in 0..size{
        point.push(p);
    }
    
    for one in report{
        let mut one= one.split_whitespace();
        let send= one.next().unwrap();
        let receive= one.next().unwrap();
        for (i, name) in id_list.iter().enumerate(){
            if name == receive{
                point[]
            }
        }        
    }

    result
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let id_list= vec!["muzi", "frodo", "apeach", "neo"];
        let report= vec!["muzi frodo","apeach frodo","frodo neo","muzi neo","apeach muzi"];
        let k= 2;
        let result= vec![2,1,1,0];
        assert_eq!(solution(id_list, report, k), result);
    }

    fn test2(){
        let id_list= vec!["con", "ryan"];
        let report= vec!["ryan con", "ryan con", "ryan con", "ryan con"];
        let k= 3;
        let result= vec![0,0];
        assert_eq!(solution(id_list, report, k), result);
    }
}