// https://doc.rust-lang.org/rust-by-example/mod/super.html

fn function(){
    println!("called `function()`");
}

mod cool{
    pub fn function(){
        println!("called `cool::function()`");
    }
}

mod my{
    fn function(){
        println!("called `my::function()`");
    }
    mod cool{
        pub fn function(){
            // my::cool::function()
        }
    }
    pub fn indirect_call(){

    }
}