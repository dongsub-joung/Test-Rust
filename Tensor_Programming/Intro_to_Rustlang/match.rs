//  ex1
fn main(){
    let p= 5;
    match p {
        n @ 1  ... 12 => println!("n: {}", n),
        n @ 13 ... 10 => println!("n: {}", n),
        _ => println!("no match"),
    }

    // let
    let p= 5;
    let n= match p {
        n @ 1  ... 12 => n,
        n @ 13 ... 10 => n, 
        _ => 0,
    };
    println!("n: {}", n);
}

