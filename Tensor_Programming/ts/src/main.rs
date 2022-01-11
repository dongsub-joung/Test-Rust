pub trait Nanana {
    pub fn nana(){}
}

impl Do for dyn Nanana {
    pub fn nana(){
        println!("na2");
    }
}


fn main() {
    // println!("Hello, world!");
    Nanana::nana();
}
