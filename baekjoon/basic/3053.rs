#[cfg(test)]
mod testing{
    use std::f64::consts::PI;

    use super::*;

    fn solution(n: f64) -> (f64, f64){
        let (mut p, mut r)= (0.0, 0.0);
        p= n*n * PI;
        r= n*n * 2.0;
        (p, r)
    }
    
    #[test]
    fn test1(){
        let input= 1;
        let result= solution(input as f64);
        assert_eq!(result, (3.141593, 2.000000));
    }

    fn test2(){
        let input= 21;
        let result= solution(input as f64);
        assert_eq!(result, (1385.442360, 882.000000));
    }

    fn test3(){
        let input= 42;
        let result= solution(input as f64);
        assert_eq!(result, (5541.769441, 3528.000000));
    }
}