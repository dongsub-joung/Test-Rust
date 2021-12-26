// https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#reference-lifetime
static NUM: i32= 18;

fn coerce_static<'a>(_: &'a i32) -> 'a i32 {
    &NUM
}

fn main(){
    {
        let static_string= "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }
    {
        let lifetime_num= 9;
        let coerced_static= coercce_static(&lifetime_num)
    }
    println!("NUM: {} stays accessible!", NUM);
}