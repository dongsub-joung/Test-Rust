use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(id_list: Vec<&str>, report: Vec<&str>, k: usize) -> Vec<usize>{
    let size= id_list.len();
    let mut result= vec![0; size];
    let check: Vec<(usize, usize)>= Vec::new();

    let size= 5_usize;
    let mut p= vec![0_usize; size];
    let mut point: Vec<Vec<usize>> = Vec::new();
    for _ in 0..size{
        point.push(p.clone());
    }
    
    let (mut a, mut b)= (0usize, 0usize);
    for one in report{
        let mut one= one.split_whitespace();
        let send= one.next().unwrap();
        let receive= one.next().unwrap();

        for (j, name) in id_list.iter().enumerate(){
            if name == &send{
                a= j;
            }
        }
        for (j, name) in id_list.iter().enumerate(){
            if name == &receive{
                b= j;
            }
        }
        point[a][b] += 1;        
    }

    for i in 0..size{
        let mut sum= 0usize;
        for j in 0..size{
            if point[i][j] > 1{
                point[i][j] = 1;
            }
            sum += point[i][j];
        }
        result.push(sum);
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

    #[test]
    fn test2(){
        let id_list= vec!["con", "ryan"];
        let report= vec!["ryan con", "ryan con", "ryan con", "ryan con"];
        let k= 3;
        let result= vec![0,0];
        assert_eq!(solution(id_list, report, k), result);
    }
}