// https://www.youtube.com/watch?v=DSZqIJhkNCM
// https://www.youtube.com/watch?v=5RPXgDQrjio

// Ex 1
mod front_of_house {
    mod hosting{
        fn add_to_waitlist(){}
    }
}

pub fn eat_at_restaurant(){
    // Absoute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Ex 2
fn serve_order(){}

mod back_of_house{

    struct Breakfast{
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String:from("peaches"),
            }
        }
    }
}

fn fix_incorrect_order(){
    cook_order();
    super::serve_order();
}

fn cook_order(){}


// Ex 3

mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}

// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;
