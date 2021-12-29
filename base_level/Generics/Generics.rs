// needness of case, if you ..
struct A;
struct Single(A);

struct SingleGen<T>(T)
fn main(){
    // concrete and explicitly take 'A'
    let _s= Single(A);

    // one struct take a variable as "let" a lot
    let _char= SingleGen('a');
    let _t    = SingleGen(A); // Uses `A` defined at the top.
    let _i32  = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.
}