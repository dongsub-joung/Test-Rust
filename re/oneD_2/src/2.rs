use std::io;

fn get_nubmers() -> Vec<f32> {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let strs= buf.split_whitespace();

    let list= strs
        .map(|f| f.parse::<f32>())
        .collect::<Result<Vec<f32>, >>()
        .unwrap();

    list
}

fn get_nubmers() -> Vec<f32> {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf.unwrap());
    let strs= buf.split_whitespace();

    let list= strs  
        .map(|f| f.parse::<f32>())
        .collect::<Result<Vec<f32>, _>>()
        .unwrap();

    list
}

fn main(){
    let list= get_nubmers();
    let n= list[0];
    let k= list[1];

    let size= (n*k) as f32 / 2.0;
    let num= size.to_string().parse<u8>().unwrap();

    println("{}", num);
}