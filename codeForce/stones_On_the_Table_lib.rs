use std::io;

#[cfg(test)]

fn logic(n: u8, s: String) -> u8{
    // [0,1,2,3]
    let mut save= 0u8;
    let mut result= 0u8;
    for element in s.as_bytes().iter(){
        if *element == save{
            result = result+1;
        }
        save= *element;
    }
    
    result
}

mod testing{
    use super::*;
    
    #[test]
    fn test1(){
        let n= 3;
        let s= String::from("RRG");

        assert_eq!(1, logic(n,s));
    }

    #[test]
    fn test2(){
        let n= 5;
        let s= String::from("RRRRR");

        assert_eq!(4, logic(n,s));
    }

    #[test]
    fn test3(){
        let n= 4;
        let s= String::from("BRBG");

        assert_eq!(0, logic(n,s));
    }


}

