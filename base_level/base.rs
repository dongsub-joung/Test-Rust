fn another_function(a: i32) {
    let b= {
        if a == 0{
            a+2
        } else {
            a+1
        }
    };
    println!("{}", b);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn tuples(){
    let d = [10, 20, 30, 40, 50];

    for element in d.iter(){
        println!("{},", element);
    }
}

fn for_rev(){
    for num in (1..4).rev(){
        println!("{}!", num);
    }
    println!("boom");
}

fn main() {
    let a= 0;
    let c= plus_one(a);

    another_function(a);
    println!("{}", a);
    println!("{}", c);

    tuples();
    for_rev();
}

