use std::io::{self, stdin};

#[cfg(test)]
fn solution(arr: Box<[usize]>, divisor: usize) -> Vec<usize>{
    let mut list= arr.clone().to_vec();
    
    for j in list.iter(){
        if j % divisor == 0{
            list.push(*j);
        } 
        list.sort();
        if list.len() == 0 {
            // usize data err.
            list.push(-1);
        }
    }

    return list;
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let boxed: Box<[usize]>= Box::new([5,9,7,10]);
        let result=solution(boxed, 5);
        
        let pass= Vec::from([5,10]);
        assert_eq!(result, pass);
    }

    #[test]
    fn test2(){
        let boxed: Box<[usize]>= Box::new([2, 36, 1, 3]);
        let result=solution(boxed, 1);
        
        let pass= Vec::from([1, 2, 3, 36]);
        assert_eq!(result, pass);
    }

    #[test]
    fn test3(){
        let boxed: Box<[usize]>= Box::new([3,2,6]);
        let result=solution(boxed, 10);
        
        let pass= Vec::from([0]);
        assert_eq!(result, pass);
    }

}