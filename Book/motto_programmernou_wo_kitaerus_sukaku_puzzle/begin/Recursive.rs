fn Recursive_fun(val: i32) -> i32{
    let mut num= val;
    if num == 0{
        return num;
    } else {
        println!("{}", num);
    }
    num= num - 1;
    return Recursive_fun(num);
}
fn main(){
    Recursive_fun(10);
    println!();
}