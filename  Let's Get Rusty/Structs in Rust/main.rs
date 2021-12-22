// https://www.youtube.com/watch?v=n3bPhdiJm9I

// Ex1
 struct User {
     username: String,
     email: String,
     sign_in_count: u64,
     active: bool
 }
 
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count= 1,
    }
}

// Ex2
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height 
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
    }
}

// If you want to add aohter "impl Rectangle", access "::" 

fn main(){
    let rect: Rectangle= Rectangle{
        width: 30,
        height: 50
    };

    println!("The are of the rectangle is {} square pixels.",
    rect.area());
}