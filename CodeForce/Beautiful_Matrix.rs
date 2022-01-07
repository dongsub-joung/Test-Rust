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
    let mut r;
    let mut c;

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
                r = i;
                c = j;
            }
        }
    }
    let th: usize = 3;
    let a= (r-3);
    let b= (c-3);

    // let value = -42i32;
    // let x = value.abs();

    // Hard to cast from usize to others integer
    // b.abs()
    // println!("{}",  +  );
}