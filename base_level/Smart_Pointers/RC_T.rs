// reference counting
// ERR code
enum List{
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main(){
    let a= Cons(5,
        Box::new(Cons(10,
            Box::new(Nil)));
    let b= Cons(3, Box::new(a));
    let c= Cons(4, Box::new(a));
}


// RC<T>
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}