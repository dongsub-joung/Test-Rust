fn main(){
    // normal
    let array= [0i32; 100];
    for i in array.iter(){
        println!("{}", &i);
    }

    // two
    let mut state = [[0u8; 4]; 6];
    for i in 0..6{
        for j in 0..4{
            state[i][j] = 1;
        }
    }
    println!("{}", state[5][3]);


    // Box
    let x: [Box<[u8]>; 3] = [
        Box::new([1, 2, 3]),
        Box::new([4]), 
        Box::new([5, 6])
    ];
    let y: &[Box<[u8]>] = &x;
    println!("{:?}", &y);

    
}