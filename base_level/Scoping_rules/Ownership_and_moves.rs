// heap allocated
fn destroy_box(c: Box<i32>){
    println!("Destrpuomg a box that contains {}", c);
}

fn main(){
    // _stack_ allocated integer
    let x= 5u32;

    // *copy*
    let y= x;

    println!("x is {}, y is {}", x,y);

    let a= Box::new(5i32);
    println!("a contains: {}", a);
    
    // *Move* `a` into `b`
    let b = a;
}


// Partial Moves
// Partial Moves
fn main(){
    #[derive(Debug)]
    struct Person{
        name: String,
        age: u8,
    }

    let person= Person{
        name: String::from("adsf"),
        age: 20,
    };

    let Person { name, ref age }= person;

    
    // 1.
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    
    // 2.
    println!("able: {}", person.age);
}