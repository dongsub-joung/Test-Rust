use std::io;

#[cfg(test)]
fn logic(s: String) -> String{
    let Y= String::from("CHAT WITH HER!");
    let N= String::from("IGNORE HIM!");
    
    if s.len() > 100{
        panic!("Err");
    }
    
    let mut num= 0u8;
    let mut cnt= 0u8;
    for (i,chr) in s.chars().enumerate(){
        for j in 0..i{
            if chr == s.as_bytes()[j] as char { num = num+1; }
        }
        if num == 0 { cnt = cnt+1; }
        num= 0;
    }

    if cnt%2 == 0 { Y }
    else { N }
}

mod testing{
    use super::*;
    
    #[test]
    fn test1(){
        let s= String::from("wjmzbmr");
        let pre= String::from("CHAT WITH HER!");
        
        assert_eq!(pre, logic(s));
    }

    #[test]
    fn test2(){
        let s= String::from("xiaodao");
        let pre= String::from("IGNORE HIM!");

        assert_eq!(pre, logic(s));
    }

    #[test]
    fn test3(){
        let s= String::from("sevenkplus");
        let pre= String::from("CHAT WITH HER!");

        assert_eq!(pre, logic(s));
    }


}

