use std::vec;

// vec.push / pop
fn chk(remain: usize, pre: usize) -> i32{
    let mut cnt= 0;
    const M: usize= 11;

    let mut width= pre.clone();
    let mut height= remain.clone();

    let mut memo= vec![vec![0i32; width+1]; height+1];

    if memo[height][width] != 0{
        // return memo[height][width];
        return memo.pop().unwrap() as i32;
    }
    if remain < 0 {
        return 0;
    }
    if remain == 0 {
        return 1;
    }
    for i in pre..M{
        cnt= cnt + chk(height-i, i);
    }

    memo[height][width]= cnt;
    return memo[height][width];
}

fn main(){
    const N: usize= 101;
    println!("{:?}", chk(N, 2));
}

// @todo 
// let m= {};
// m[[2,2]]= 0;
// console.log(m);

// var array = [[{path: 'path/image', preview: 'blob:imagepreview'}]]
// console.log(array[0][0]