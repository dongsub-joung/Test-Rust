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