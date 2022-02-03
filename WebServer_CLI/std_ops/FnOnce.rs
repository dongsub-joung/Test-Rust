// https://doc.rust-lang.org/std/ops/trait.FnOnce.html
pub trait FnOnce<Args>{
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

fn consum_with_relish<F>(func: F)
    where F: FnOnce() -> String
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}