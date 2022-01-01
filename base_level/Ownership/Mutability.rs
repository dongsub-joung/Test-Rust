// doc.rust-lang.org
fn main(){
    let immutable_box= Box::new(5u32);

    // err : 
    *immutable_box= 4;

    let mut mutable_box= immutable_box;

    // ok
    *mutable_box= 4;   
}