use std::io::{self, Read};

#[cfg(test)]

fn get_list() -> Vec<u8> {
    let mut buf= String::new();
 
    io::stdin().read_to_string(&mut buf).unwrap();

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
    fn one() {
        let list= vec![10,8,6,4,2,1];

        assert_eq!(
            list
            ,get_list()
        );
    }
}