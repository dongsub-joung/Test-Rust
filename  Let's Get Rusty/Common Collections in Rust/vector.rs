// https://www.youtube.com/watch?v=Zs-pS-egQSs

fn main(){
    let a= [1,2,3];
    let mut v:Vect<i32>= Vect::new();

    {
        // convenience express
        let v2= vec![1,2,3];
        // Lifetime
    }

    let V= vec![1,2,3,4,5];
    let thired= &V[2];
    match V.get(2){
        Some(thired) => println!("The third element is {}", thired),
        None         => println!("There is no third element."),
    }
}    


// Ex2
fn main(){
    let mut v: vec![1,2,3,4,5];

    // smart pointer (dereference)
    for i in &mut v{
        *i += 50;
    }

    for i in &v{
        println!("{}", i);
    }
}

// Ex 3
fn main(){
    enum SpreadsheetCell{
        int(i32),
        Floadt(f64),
        Text(String),
    }

    let row= vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Floadt(10.12),
    ]

    match &row[1]{
        SpreadsheetCell::Int(i) => println!("{}", i),
        _                       => println!("Not a integer!")
    }
}

// Ex 4 
use std::collections:HashMap;
fn main(){
    let blue= String::from("Blue");
    let yellow= String::from("Yellow");

    let mut score= HashMap::new();

    score.insert(blue, 10);
    score.insert(yellow, 50);
}

// Ex 5
fn main(){
    let text= "hello world wonderful world";

    let mut map= HashMap::new;

    for word in text.split_whitespace() {
        let count: &mut i32= map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// https://www.youtube.com/watch?v=Zs-pS-egQSs&t=428s