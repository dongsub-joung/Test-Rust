fn main(){
    let mut v= vec![5, 2, 9, 1, 6];
    let length= 5_usize;
    let mut tmp= 0_usize;
    
    println!("{:?}", v);

    for i in 0..length{
        for j in 0..length-i-1{
            if v[j]>v[j+1]{
                tmp= v[j+1];
                v[j+1]= v[j];
                v[j]= tmp;
            }
        }
    }

    println!("{:?}", v);
}
