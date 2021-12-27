// Ex

fn main(){
    let v1= vec![1,2,3];
    for element in v1.iter(){
        match element {
            integer: u32 => prinln!("Integer"),
            None         =>  prinln!("Non- Integer"),
//             Option
            some(1) => prinln!("1"),
            some(2) => prinln!("2"),
            _       => prinln!("Over 2"),
            // None    => prinln!("No value"),
        }
    }
}

fn main(){
    let text= "hello world wonderful world";
    let mut map= HashMap::new;

    for word in text.split_whitespace(){
        let count= map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
