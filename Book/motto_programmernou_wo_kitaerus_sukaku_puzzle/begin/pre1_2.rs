// Dead, dont work code 

// vec.push / pop
fn chk(remain: i32, pre: i32) -> []{
    let mut memo= [0i32];
    let mut cnt= 0;

    if memo[remain,pre] != None {
        return memo[remain,pre];
    }
    if remain < 0 {
        return cnt;
    }
    if remain == 0 {
        return 1;
    }

    return memo[remain, pre]= cnt;
}

fn main(){
    const N: i32= 100;
    chk(N, 2);
}