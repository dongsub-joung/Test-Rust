// Some test code
struct User {
    id: String,
    pw: String,
    private_number: u32,
}

struct PrivateNumberHashed{
    first: String,
    second: String,
}

fn addArray(element: u32){
    (element)
}
fn hashing(elment: u32) -> &str {
    elment
}

enum Psersonal{
    first(u32),
    second(&str),
}

fn handlingPN(users: &User) ->  {
    let private_number= &users.private_number;
    const FIRST= private_number[..6]
    const SECOND= private_number[7..]

//     @todo sovling ownership problems
    // arraying "FIRST"
    let Pserosnal::first= addArray(&FIRST);
    // Hasing "SECOND"
    let Pserosnal::second= hashing(&SECOND);
    
//     map(&array, &hashing);
}

fn main(){
/*
    let id: &'static str= inputeId();
    let pw: &'static str= inputePw();
    let PN: &'static str= inputePN();*/
    let user001= {"asdf", "qwerqwer", 12312312312}
    handlingPN(user001)l;
}


// Main Example
fn area(dimensions: (u32, u32)) -> u32 {
    // length: u32, width: u32
    // length*width
    dimensions.0 * dimensions.1
}

fn main() {
    // let length1= 50;
    // let width1= 30;
    let rect1= (50,30);
s
    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}


// After refectoring
#[derive(Debug)]
struct Rectangle {
    lenght: u32,
    width:  u32,
}
// fn area(rectangle: &Rectangle) -> u32 {
//   rectangle.length * rectangle.width
// }

fn main() {
    // let length1= 50;
    // let width1= 30;
    let rect1= Rectangle{length: 50, width: 30};

    // Function
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    // #[derive(Debug)]
     println!("rect1 is {:#?}", rect1);
}

