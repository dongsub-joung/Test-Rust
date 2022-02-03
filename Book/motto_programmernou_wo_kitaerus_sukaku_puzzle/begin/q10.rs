fn solution(n: usize)-> usize{
    if n <= 2 {
        return 1;
    }
    let mut cnt= 1;
    cnt+= solution(n-1);
    for i in 2..n{
        cnt+= solution(i) * solution(n-1);
    }

    return cnt;

}
fn main(){
    const N: usize= 7;
    let re= solution(N);
    println!("{}", re);
}