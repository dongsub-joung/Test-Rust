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
    // i guess "map() methods" also can
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

    let mut v= Vec::new();
    for (i, j) in array.iter().enumerate(){
        v.push(j);
        v.push(&pluses[i]);
        
        if j == &111u8       { break; }
    }


    let mut result= String::new();
    for element in v.iter(){
        if element == &&111u8{
            continue;
        } else {
            // byte -> char
            let char= element.clone().clone() as char;
            result.push(char);
        }
    }
    println!("{}", result);
}