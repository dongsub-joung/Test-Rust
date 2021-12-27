// Path
mod front_of_house {
    mod hosting{
        fn add_to_waitlist(){}
    }
}

pub fn eat_at_restaurant(){
    cate::front_of_house::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

// struct, impl (init structure) and super()
fn serve_order(){}

mod back_of_house{
    struct Breakfast{
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// public modul "use"
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;