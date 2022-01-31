fn main(){
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut num= buf.trim().parse::<usize>().unwrap();
    let mut m= 2;
    while num != 1 {
        if num % m == 0{
            println!("{}", m);
            num= num / m
        } else {
            m += 1;
        }
    }
}