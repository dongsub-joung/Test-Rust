// Basic
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny  => 1,
        Coin::Nickel => 5,
        Coin::Dime   => 10,
        Coin::Quarter=> 25,
    }
}

// Option<T>
fn plus_one(x:Option<i32>) -> Option<i32>{
    None    => None,
    Some(i) => Some(i+1),
}

let five= Some(5);
let six= plus_one(five);
let none= plus_one(None);

// fit "else" == "_"