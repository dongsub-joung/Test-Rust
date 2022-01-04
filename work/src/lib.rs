use std::io;

#[cfg(test)]
mod tests{
    use super::*;

    fn get_win() -> i8 {
        let A: i8;
        let B: i8;
        let C: i8;
        
        let mut buf= String::new();
        io::stdin().read_line(&mut buf).unwrap();
        
        let mut list= buf.split_whitespace();
    
        let mut a= list.next().unwrap();
        A= a.trim().parse().unwrap();
        
        let mut b= list.next().unwrap();
        B= b.trim().parse().unwrap();
    
        let mut c= list. next().unwrap();
        C= c.trim().parse().unwrap();
    
        A+B+C
    }

    #[test]
    fn one(){
        assert_eq!(3, get_win());
    }
}    
