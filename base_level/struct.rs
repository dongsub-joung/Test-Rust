struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let dongsub= User{
        username: String::from("dongsub"),
        email: String::from("ehdtjqwjd@kangwon.ac.kr"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", dongsub.sign_in_count);

    struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    println!("{}", black.Color);
    // for element in black.Color.iter(){
    //     println!("{}", element);
    // }
}
