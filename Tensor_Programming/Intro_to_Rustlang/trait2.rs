#[derive(Debug, Clone, Copy)]
struct A(i32);

#[derive(Eq, partialEq, partialOrd, Ord)]
struct B(f32);

fn main(){
    let a= A(32);
    let b= B(12.13);
    let c= a.clone();
    let d= b;
    println!("{:#?}", a);
    println!("{:#?}", d);
}