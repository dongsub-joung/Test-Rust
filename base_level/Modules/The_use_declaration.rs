// https://doc.rust-lang.org/rust-by-example/mod/use.html

use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType,
};

fn main(){
    my_first_function();
    println!("Entering block")
    {
        use cate::deeply::nested::function;
        function();

        println!("Leaving block");
    }
    function();
}