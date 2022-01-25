use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(arr_1a: Box<[usize]>, arr_1b: Box<[usize]>, arr_2a: Box<[usize]>, arr_2b: Box<[usize]>) -> (Vec<usize>, Vec<usize>){
    let mut a= Vec::new();
    let mut b= Vec::new();
    for (i,j) in arr_1a.iter().enumerate(){
        a.push(arr_2a[i] + j);
    }
    for (i,j) in arr_1b.iter().enumerate(){
        b.push(arr_2b[i] + j);
    }

    return (a, b);
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let (arr1_a, arr1_b)= 
            (Box::new([1usize,2]),
            Box::new([2,3]));

        let (arr2_a, arr2_b)= 
            (Box::new([3usize,4]),
            Box::new([5,6]));

        let result_a: Vec<usize>= Vec::from([4,6]);
        let result_b: Vec<usize>= Vec::from([7,9]); 
        let (sol1, sol2)= 
            solution(arr1_a, arr1_b, arr2_a, arr2_b);
        assert_eq!(sol1, result_a);
        assert_eq!(sol2, result_b);
    }
}