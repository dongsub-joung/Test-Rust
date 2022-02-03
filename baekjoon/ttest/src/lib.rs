// use std::{io::{self, stdin}};

// // f64, Rounding
// #[cfg(test)]
// fn solution(m: i32, n: i32) -> (i32, i32){
//     let mut v2: Vec<i32>= Vec::new();
//     let mut sum= 0;
//     let mut min= 0;

//     for i in m..n+1{
//         for j in 2..i+1{
//             if j == i{
//                 v2.push(i);
//             }
//             if i % j == 0 {
//                 break;
//             }
//         }
//     }

//     if v2.len() != 0 {
//         sum= v2.iter().sum();
//         min= v2.iter().min().unwrap().clone();
//     } else {
//         sum= -1;
//         min= -1;
//     }

//     (sum, min)
// }

// mod testing{
//     use super::*;

//     #[test]
//     fn test1(){
//         let m= 60;
//         let n= 100;
//         assert_eq!(solution(m,n), (620, 61));
//     }

//     #[test]
//     fn test2(){
//         let m= 64;
//         let n= 65;
//         assert_eq!(solution(m,n), (-1, -1));
//     }
// }