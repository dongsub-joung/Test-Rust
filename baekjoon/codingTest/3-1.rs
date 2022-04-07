fn binary_search(v: Vec<usize>, length: usize, target: usize) -> usize{
    let mut start= 0_usize;
    let mut end= length-1;
    let mut mid;
    
    loop {
        mid= (start+end) / 2;
        if end >= start{
            break 42;
        }
        
        if v[mid] == target{
            return 1;
        } else if v[mid] > target{
            end= mid - 1;
        } else {
            start= mid + 1;
        }
    }
}
fn main(){
    let v= vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let length= 10_usize;
    let target= 6_usize;

    let result= Some(binary_search(v, length, target));
    match result {
        None => println!("None exist"),
        Some(i) => println!("Search it"),
    }
}
