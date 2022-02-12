#[cfg(test)]

mod testing{
    use super::*;

    fn solution(n: i32, input: &str) -> String{
        let mut result= String::new();
        let mut v= Vec::new();
        for i in input.split(""){
            let mut one= String::new();
            for _ in 0..n{
                one.push_str(i);
            }
            v.push(one);
        }

        for i in v{
            result.push_str(&i);
        }

        result
    }
    

    fn test1(){
        let input= "ABC";
        let n= 2;
        let solving= solution(n, input);
        let result= "AAABBBCCC";
        assert_eq!(solving, result.to_string());
    }

    fn test2(){
        let input= "/HTP";
        let n= 3;
        let solving= solution(n, input);
        let result= "/////HHHHHTTTTTPPPPP";
        assert_eq!(solving, result.to_string());
    }
}