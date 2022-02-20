fn factorial(n: u128) -> u128{
    match n {
        0 | 1 => 1,
        _ => factorial(n-1)*n,
    }
}
fn main(){
    println!("{}", factorial(10));
}