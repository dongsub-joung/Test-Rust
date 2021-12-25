// https://www.koderhq.com/tutorial/rust/smart-pointer/
// https://doc.rust-lang.org/std/ops/trait.Deref.html
// C++ STL

// Use Deref from standard library  
use std::ops::Deref;

struct DerefExample<T>{
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target= T;
    fn deref(&self) -> &Self::Target{
        &self.value
    }
}

let x= DerefExample { value: 'a'};
assert_eq!('a', *x);