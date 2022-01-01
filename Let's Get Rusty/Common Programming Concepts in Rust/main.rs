fn main(){
    // Tuple
    // compound types
    let tup: (&str, i32) = ("Let's Get Rusty!", 100_100);
    let (chanel: &str, sub_count: i32)= tup;
    let sub_count: i32= tup.1;
    println!("{}", &sub_count);

    let byte: [i32; _]= [0; 8];

    // control flow  
    let condition: bool= true;
    let number: i32= if condition { 5 } else { 6 }; 

    let a: [i32; _]= [10,20,30,40,50,60,70,80,90];
    for element: i32 in a.iter(){
        println!("{}", element);
    }

    // let a: [i32; 9];
    // a= [10,20,30,40,50,60,70,80,90];
    // for element in a.iter(){
    //     println!("{}", element);
    // }
}