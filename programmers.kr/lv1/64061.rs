fn solution(board: [[usize; 5]; 5], mvs: Box<[usize]>) -> i32{
    // let stack_list= Box::new([]);
    let mut stack_list: Vec<usize>= Vec::new();
    let mut answer= 0;

    for i in mvs.iter(){
        for j in 0..board.len(){
            if board[j][i-1] != 0{
                // type err
                stack_list.append(&mut board[j][i-1]);
                board[j][i-1]= 0;

                if stack_list.len() > 1 {
                    if stack_list[-1] == stack_list[-2]{
                        // index -1
                        stack_list.pop();
                        stack_list.pop();
                        answer += 2;
                    }
                }
                break;
            }
        }
    }

    answer
}

fn main(){
    let board = [
        [0,0,0,0,0],
        [0,0,1,0,3],
        [0,2,5,0,1],
        [4,2,4,4,2],
        [3,5,1,3,1]
        ];
    let moves = [1,5,3,5,1,2,1,4];
    println!(solution(board,moves));
}