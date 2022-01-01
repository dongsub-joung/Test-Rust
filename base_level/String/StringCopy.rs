fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let aa= "Hi";
    let ab= "Ho";

    // Moving (a mapping)
    let b= {
        aa
    };

    // cloning
    let c= {
        ab.clone()
    };

    let d= &aa;

    // borrowing
    let mut ba= String::from("yo");
    ba.push_str("n a");
    let e= &mut ba;
    // e.push_str("an");

    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);

    let values= first_word(e);
    println!("{}", values);

    let cut_of= &e[0..1];
    println!("{}", cut_of);

    e.clear();
    println!("AF clear: {}", e);

    let last: &'static str= "Someone";
    println!("{}", last);

}
