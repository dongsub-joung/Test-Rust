// https://www.youtube.com/watch?v=YfIlkghucQ4&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW&index=4

ust std::fmt;

struct Object{
    a: u32,
    b: u32,
}

impl Object {
    fn new(w: u32, h: u32) -> Object{
        Object {
            w, h,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.w, self.h)
    }
}