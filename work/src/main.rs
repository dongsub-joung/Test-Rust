use std::io;

fn get_numbers() -> Vec<f32> {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let strs= buf.split_whitespace();

    let list =strs
        .map(|f| f.parse::<f32>())
        .collect::<Result<Vec<f32>, _>>()
        .unwrap();

    list
}

fn main() {
    let list= get_numbers();
    let n= list[0];
    let k= list[1];
    
    let size: f32= (n*k) / 2.0;
    let num: u8= size.to_string().parse().unwrap();
    
    println!("{}", num);
}
