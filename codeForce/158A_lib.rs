use std::io::{self, Read, BufRead};

#[cfg(test)]
fn get_numbers() -> (usize, usize){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut list= buf.split_whitespace();

    let num1= list.next().unwrap();
    let n= num1.parse().unwrap();

    let num2= list.next().unwrap();
    let k= num2.parse().unwrap();
    
    (n, k)
}

fn get_list() -> Vec<u8> {
    let mut buf= String::new();
 
    io::stdin().read_line(&mut buf).unwrap();

    let list= buf
        .split_whitespace()
        .map(|f| f.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    list
}


mod tests{
    use super::*;

    #[test]
    #[ignore = "reason"]
    fn number() {
        assert_eq!((1,2), get_numbers());
    }

    #[test]
    fn list(){
        let v= vec![1, 2, 3];
        assert_eq!(v, get_list());
    }
}