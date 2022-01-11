use std::io;

fn get_n() -> i8 {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: i8= buf.trim().parse().unwrap();

    num
}

fn main(){
    let mut state = [[0i8; 6]; 6];
    let size: usize= 6;
    let mut r: u128;
    let mut c: u128;

    {
        for i in 1..size{
            for j in 1..size{
                state[i][j]= get_n();
            }
        }
    }
    for i in 1..size{
        for j in 1..size{
            let val= state[i][j];
            if val == 1{
                r = i.try_into().unwrap();
                c = j.try_into().unwrap();

                let r= (r-3).abs();
                let c= (c-3).abs();
            }
        }
    }
    println!("{}",  r + c);
}