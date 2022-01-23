use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(mut list: Box<[i32]>) -> Box<[i32]>{

    // use of unstable library feature 'slice_partition_dedup' see issue #54279
    let (dedup, duplicate)= list.partition_dedup();
    
    // the size for values of type `[i32]` cannot be known at compilation time the trait `Sized` is not implemented for `[i32]`
    let a= Box::new(*dedup);
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let input= Box::new([1,1,3,3,0,1,1]);
        let result= Box::new([1,3,0,1]);
        assert_eq!(solution(input), result);
    }

    #[test]
    fn test2(){
        let input= Box::new([4,4,4,3,3]);
        let result= Box::new([4,3]);
        assert_eq!(solution(input), result);
    }

}