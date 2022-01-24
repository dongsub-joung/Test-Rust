use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(mut list: Box<[i32]>) -> Box<[i32]>{

    // use of unstable library feature 'slice_partition_dedup' see issue #54279
    let (dedup, duplicate)= list.partition_dedup();
    
    // the size for values of type `[i32]` cannot be known at compilation time the trait `Sized` is not implemented for `[i32]`
    let a= Box::new(*dedup);
}

fn solution2(list: Box<[usize]>) -> Vec<usize>{
    let mut pre= 0usize;
    let mut next= 0usize;
    let mut result= Vec::new();
    for (i, j) in list.iter().enumerate(){
        pre= *j;
        next= list[i+1];
        if pre==next{
            result.push(pre);
        }
    }

    result
}

fn solution3(list: Box<[usize]>) -> Vec<usize>{
    let mut vs= Vec::new();
    let mut temp=0usize;
    for i in list.iter(){
        if temp == *i {
            vs.push(temp);
        }
        temp= *i;
    }

    let mut temp2= 0usize;
    let mut vss: Vec<usize>= Vec::new();
    for element in vs.iter(){
        if temp2 != *element{
            vss.push(*element);
        } else{
            continue;
        }
        temp2= *element;
    }    

    vss
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let input= Box::new([1,1,3,3,0,1,1]);
        let result= Vec::from([1,3,0,1]);
        assert_eq!(solution3(input), result);
    }

    #[test]
    fn test2(){
        let input= Box::new([4,4,4,3,3]);
        let result= Vec::from([4,3]);
        assert_eq!(solution3(input), result);
    }

}