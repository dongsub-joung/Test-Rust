// Borrowing
fn eat_box_i32(boxed_i32: Box<i32>) -> Box<i32> {
    println!("Destroying box that contains {}", boxed_i32);
    boxed_i32
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) -> &i32{
    println!("This int is: {}", borrowed_i32);
    borrowed_i32
}

fn main() {
    // Create a boxed i32, and a stacked i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    let a= borrow_i32(&boxed_i32);
    let b= borrow_i32(&stacked_i32);
    eat_box_i32(boxed_i32);
    // eat_box_i32(boxed_i32);
}



// Mutability
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book{
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}



// Aliasing


// The ref pattern