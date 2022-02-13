#[cfg(test)]

mod testing{
    use super::*;

    fn inputing(a: &str, b: &str) -> (usize, usize){
        let a= a.chars().rev().collect::<String>();
        let b= b.chars().rev().collect::<String>();
        let a: usize= a.trim().parse().unwrap();
        let b: usize= b.trim().parse().unwrap();
        
        (a, b)
    }

    fn solution(a: usize, b: usize) -> usize{
        let mut result= 0_usize;
        if a > b {
            result= a;
        } else {
            result= b;
        }

        result
    }
    

    fn test1(){
        let (a,b)= inputing("734","893");
        assert_eq!(solution(a, b), 437);
    }
    fn test2(){
        let (a,b)= inputing("221","231");
        assert_eq!(solution(a, b), 132);
    }
    fn test3(){
        let (a,b)= inputing("839","237");
        assert_eq!(solution(a, b), 938);
    }
}