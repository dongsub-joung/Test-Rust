use std::{io::{self, stdin}};

#[cfg(test)]
fn solution(mut board: [[usize; 5]; 5], moves: Vec<usize>) -> i32{
    // let stack_list= Box::new([]);
    let mut stack_list: Vec<&mut usize>= Vec::new();
    let mut answer= 0;

    for i in moves.iter(){
        let mut board= board.clone();
        for j in 0..board.len(){
            if board[j][i-1] != 0{
                // Two mutable value complie trouble
                stack_list.push(&mut board[j][i-1]);
                board[j][i-1]= 0;

                if stack_list.len() > 1 {
                    // let copy= stack_list
                    if  stack_list.pop().unwrap() == stack_list.pop().unwrap(){
                        // index -1
                        // stack_list.pop();
                        // stack_list.pop();
                        answer += 2;
                    }
                }
                break;
            }
        }
    }

    answer
}
mod testing{
    use super::*;

    #[test]
    fn test1(){
        let board = [
            [0,0,0,0,0],
            [0,0,1,0,3],
            [0,2,5,0,1],
            [4,2,4,4,2],
            [3,5,1,3,1]
            ];
        let moves = [1,5,3,5,1,2,1,4].to_vec();
        let result= 4;
        assert_eq!(solution(board, moves), result);
    }

}