use std::io;

fn inputing() -> (u8, u8){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let list= buf.trim().as_bytes();

    return (list[0], list[2]);
}


fn convering(a: u8, b: u8) -> (u16, u16){
    return (conver_num(a), conver_num(b));
}
fn conver_num(chr: u8) -> u16{
    match chr {
        48 => { return 0; }
        49 => { return 1; }
        50 => { return 2; }
        51 => { return 3; }
        52 => { return 4; }
        53 => { return 5; }
        54 => { return 6; }
        55 => { return 7; }
        56 => { return 8; }
        57 => { return 9; }
        _  => { return chr as u16;}
    }
}

fn main() {
    let (val_a, val_b)= inputing();
    let result= convering(val_a, val_b);
    println("{}", result);
}