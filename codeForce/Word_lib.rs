use std::io::{self, stdin};

#[cfg(test)]
fn stuff(buf: String) -> String{
    if buf.len() > 100 {
        panic!("1 to 100")
    }

    let list= buf.as_bytes();
    let mut cnt:f64= 0.0;
    for i in list.iter(){
        let lowper_line: u8= 95;
        if i > &lowper_line{
            cnt += 1.0;
        }
    }

    let divid= (list.len() as f64 / 2.0) as f64;
    let remain= divid - cnt;
    if remain <= 0.0{
        buf.to_lowercase()
    } else {
        buf.to_uppercase()
    }
}

mod testing{
    use super::*;

    #[test]
    fn test1(){
        let result= stuff(String::from("HoUse"));
        assert_eq!(result, String::from("house"));
    }
    #[test]
    fn test2(){
        let result= stuff(String::from("ViP"));
        assert_eq!(result, String::from("VIP"));
    }
    #[test]
    fn test3(){
        let result= stuff(String::from("maTRIx"));
        assert_eq!(result, String::from("matrix"));
    }
}