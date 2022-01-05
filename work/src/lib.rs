// use std::io::{self, Read};

// #[cfg(test)]

// fn get_list() -> [u32; 1000] {
//     let mut buf= String::new();
//     let mut list= [0; 1000];

//     // *Infinit loop* 
//     io::stdin().read_to_string(&mut buf).expect("msg");
//     // this
    
//     let list_strs= buf.split_whitespace();
//     for (index, element) in list_strs.enumerate(){
//         list[index]= element.trim().parse().expect("msg");
//     }

//     list
// }

// mod tests{
//     use super::*;

//     #[test]
//     fn one() {
//         let mut list= [0; 1000];
//         list[0]= 10;
//         list[1]= 8;
//         list[2]= 6;
//         list[3]= 4;

//         assert_eq!(
//             list
//             ,get_list()
//         );
//     }
// }