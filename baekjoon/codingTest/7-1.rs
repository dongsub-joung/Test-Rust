fn is_prime(n: usize) -> bool{
    let mut result= false;

    for i in 2..n{
        if n%i == 0{
            result= false;
            break;
        } else {
            result= true;
        }
    }

    result
}

fn print_prime(n: usize){
    if is_prime(n){
        println!("prime");
    } else {
        println!("Normal");
    }
}
fn main(){
    print_prime(97);
    print_prime(100);
}