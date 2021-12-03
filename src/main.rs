fn main() {
    capacity001();
    capacity002();
}

fn capacity001() {
    let mut s= String::new();
    println!("{}", s.capacity());

    for _ in 0..5{
        s.push_str("hello");
        println!("{}", s.capacity());
    }
    // 0,8,16,16,32,23 
}

fn capacity002(){
    let mut s= String::with_capacity(25);
    println!("{}", s.capacity();
    
    for _ in 0..5{
        s.push_str("hello");
        println!("{}", s.capacity());
    }
}

