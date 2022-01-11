struct A;
struct B;
#[derive(Debug)]
struct AB;
#[derive(Debug)]
struct BA;

impl ops::Add<B> for A {
    type Output= AB;

    fn add(self, _rhs: B) -> AB{
        AB
    }
}

impl ops::Add<A> for B {
    type Output= BA;

    fn add(self, _rhs: A) -> BA {
        BA
    }
}

fn main(){
    println!(":?", A+B)
    println!(":?", B+A)
    // println!(":?", B+B)// err
}