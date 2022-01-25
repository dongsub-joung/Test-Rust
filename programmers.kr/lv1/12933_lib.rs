use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(n: usize) -> usize{
    let strs= n.to_string();
    let str: Vec<&str>= strs.split("").collect();
    let mut v: Vec<usize>= Vec::new();
    for i in str{
        if i == ""{
            continue;
        } else {
            let val= i.parse::<usize>().expect("first");
            v.push(val);
        }
    } 

   v.sort_by(|x, y| y.cmp(&x)); 

   println!("{:?}", &v);

   let mut result_str= String::new();
   for element in v.iter(){
        result_str.push_str(&element.to_string());
   }

   let result_int= result_str.parse::<usize>().expect("second");

   println!("{:?}", &result_int);
   
   result_int
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let input= 118372usize;
        let result= 873211usize;
        assert_eq!(solution(input), result);
    }
}