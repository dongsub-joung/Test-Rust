fn main(){
    let x= vec![1,2,3];

    let equal_to_x= |z| z==x;

    println!("cna't use x here: {:?}", x);

    let y= vec![1,2,3];

    assert!(equal_to_x);
}