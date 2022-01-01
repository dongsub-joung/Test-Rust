// Eng Doc
enum webEvent {
    PageLad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    click{ x: i64, y: i64 },
}



// KR Doc
fn main(){
    enum IpAddrkind{
        V4(u8,u8,u8,u8),
        V6(String),
    }
// After
    let home= IpAddr::V4(127, 0, 0, 1);
    let loopback=

// before
    struct IpAddr {
        kind: IpAddrkind,
        address: String,
    }

    let home= IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback= IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

}
