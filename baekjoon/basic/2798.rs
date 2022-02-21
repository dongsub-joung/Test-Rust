fn main(){
    let mut sum= 0_i32;
    let mut min= 9999999_i32;
    let mut goal= 0_i32;
    
    // (num, target)= inputing();
    let num= 5_usize;
    let target= 21;
    
    let v: Vec<i32>= vec![5,6,7,8,9];
    // let mut v: Vec<usize>= inputing2;
    
    for i in 0..num-2{
        for j in i+1..num-1{
            for k in j+1..num{
                sum= v[i] + v[j] + v[k];
                if (target - sum < min) && (target - sum >= 0){
                    min= target-sum;
                    goal= sum;
                }
            }
        }
    }

    println!("{}", goal);
}