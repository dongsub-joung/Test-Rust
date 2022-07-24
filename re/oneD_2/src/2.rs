use std::io;

fn get_nubmers() -> Vec<f32> {
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let strs= buf.split_whitespace();

    let list= strs
        .map(|f| f.parse::<f32>())
        .collect::<Result<Vec<f32>, >>()
        
}