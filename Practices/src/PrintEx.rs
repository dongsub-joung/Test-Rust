pub mod PrintEx{
    struct StringT{
        str_thing: String,
    }

    pub impl StringT{
        pub fn new(bla: String){
            println!("{}", bla)
        }

        pub fn slicing(bl: String) -> &str{
            let min= 0;
            let max= bl.len();
        }
    }
}