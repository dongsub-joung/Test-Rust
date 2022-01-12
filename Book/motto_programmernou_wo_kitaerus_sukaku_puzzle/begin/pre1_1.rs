fn chk(remain: i32, pre: i32) -> i32{
    const M: i32= 10;
    let mut cnt= 0;
    
    if remain < 0{
        // panic!("err");
        return 0;
    } else if remain == 0 {
        // panic!("err");
        return 1;
    } else {
        for i in pre..M+1{
            cnt =  cnt + chk(remain-i, i);
        }
    }
    
    cnt
}

fn main(){
    const N: i32= 100;
    let result= chk(N,2);
    println!("{}", result);
}