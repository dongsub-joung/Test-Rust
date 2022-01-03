struct Empty;
struct Null;

// A trait generic over 'T'
trait DoubleDrop<T> {
    fn doubledrop(self, _: T );
}

impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _:T ){}    
}

fn main(){
    let empty= Emtpy;
    let null= Null;

    // Deallocate 'empty' and 'null'.
    empty.double_drop(null);
}