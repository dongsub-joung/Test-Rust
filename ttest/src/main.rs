use std::io::{self, stdin};

fn inputing() -> String {
    let mut word= String::new();
    stdin().read_line(&mut word).unwrap();
    word.trim().to_string()
}

fn main(){
    const HIGHTEST: u8= 111;
    let word= inputing();
    let mut array= [HIGHTEST; 100];
    let mut pluses=[HIGHTEST; 100];

    let mut a_idx= 0usize;
    let mut p_idx=0usize;
    for (index, char) in word.bytes().enumerate(){
        if index%2 == 1 {
            pluses[p_idx]= char;
            p_idx += 1;
        } else {
            array[a_idx]= char;
            a_idx += 1;
        }
    }

    array.sort();

    // i guess "map() methods" also can
    let mut v= Vec::new();
    const PLUSE: u8= 43;
    for (i, j) in array.iter().enumerate(){
        if i%2 == 0{
            v.push(j);
        } else {
            v.push(&PLUSE);
            v.push(j);
        }
        if j == &111u8 {
            break;
        }
    }

    // let mut result: &str= "";
    for element in v.iter(){
        if element == &&111u8{
            continue;
        } else {
            // byte -> char
            let char= element.to_string();
            println!("{}", element);
        }
    }

    // println!("{:?}", v);
}