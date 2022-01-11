// https://www.youtube.com/watch?v=B9cHhfspDDE&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW&index=9
// 07:17

// https://users.rust-lang.org/t/calling-self-method-inside-a-mut-self-method/45752

impl Drop for A {
    fn drop(&mut self){
        printlnt!("dropped {}", self.a);
    }
}

fn main() {
    let a= A{ a: String::from("A")};
    {
        let b= A{ a: String::from("B") };

        {
            let c= A{ a: String::from("C")};

            println!("leaving inner scope 2");
        }
        println!("leaving inner scope 1");
    }

    drop(a);
    println!("program ending");
}