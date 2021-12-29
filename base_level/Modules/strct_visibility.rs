// https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html

mod my{
    pub struct OpenBox<T>{
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T>{
        contents: T,
    }

    impl <T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T>{
            ClosedBox {
                contents: contents,
                // contents,
            }
        }
    }
}

fn main(){
    let open_box= my::OpenBox{contents: "public info"};
}