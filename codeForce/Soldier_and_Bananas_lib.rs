use std::io::{self, stdin};

#[cfg(test)]
fn inputing() -> (usize, usize, usize){
    let mut a= "";
    let mut b= "";
    let mut c= "";

    let mut buff= String::new();
    stdin().read_line(&mut buff).unwrap();
    let str_list =&buff.trim().split_whitespace();

    for (i, j) in str_list.clone().enumerate(){
        if i == 0 {
            a= j;
        } else if i == 1{
            b= j;
        } else if i == 2{
            c= j;
        }
    }
    let a1:usize = a.parse().unwrap();
    let b1:usize = b.parse().unwrap();
    let c1:usize = c.parse().unwrap();

    (a1, b1, c1)
}

mod testing{
    use super::*; 

    #[test]
    #[ignore = "reason"]
    fn test1(){
        let (a,b,c)= inputing();
        assert_eq!((1,2,3), (a,b,c));
    }

    #[test]
    fn test2(){
        let (k, n, w)= (3, 17, 4);
        let mut pice: Vec<usize>= Vec::new();
        let mut result= 0;
        for i in 1..w+1{
            pice.push(i*k);
        }
        for i in pice.iter(){
            result+= i;
        }
        assert_eq!(13, (result - n));
    }
}
