use std::io::{self, Read, BufRead};

#[cfg(test)]
fn limit() -> usize{
    let mut buff= String::new();
    
    let std= io::stdin();
    let mut handle= std.lock();

    handle.read_line(&mut buff).unwrap()
}

fn word() -> String {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

mod tests{
    use super::*;

    #[test]
    #[ignore = "reason"]
    fn one() {
       assert_eq!(4, limit());
    }

    #[test]
    fn two(){
        let w= word();

        assert_eq!("ab\n", w);
    }

}