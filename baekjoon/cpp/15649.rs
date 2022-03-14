use std::io::{self, stdin};
static N: i32= 3;
static M: i32= 1;
static MAX: usize= 9;

fn main(){
    dfs(0);
}

fn dfs(k: i32){
    let mut arr: Vec<usize>= Vec::new();
    let mut visited: Vec<bool>= vec![false; MAX];

    if k == M{
        for i in 0..M{
            let i= i as usize;
            println!("{:?}\n", arr[i]);
        } 
    } else {
        for i in 1..N+1{
            let i= i as usize;
            let k= k as usize;
            if !visited[i]{
                visited[i]= true;
                arr[k]= i;
                
                let k= k as i32;
                dfs(k+1);
                visited[i]= false;
            }
        }
    }
}
