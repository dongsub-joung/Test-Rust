
use std::io::{self, stdin};

fn inputing() -> i64{
    let mut buf= String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n= buf.split_whitespace().next().unwrap();
    
    n.parse::<i64>().unwrap()
}

fn primeNumber(){
    let n= inputing();
    if n < 2{
        println!("Not Prime");
    }

    if n == 2{
        println!("2 is prime");
    }

    let interger= (n as f64).sqrt() as i64;
    for i in 2..interger{
        if n % i == 0{
            println!("Not Prime");
        }
    }

    println!("{} is Prime Number", n);

}

fn eratos(){
    let mut prime: Vec<bool>= Vec::new();
    let number= inputing();

    for i in prime{
        println!("{}", i);
    }
}

fn main(){
    let mut v: Vec<i64>= Vec::new();
    loop {
        let num= inputing();
        if  num == 0{
            break;
        }
        v.push(num);
    }

    // 4948
    let mut result: Vec<usize>= Vec::new();
    for i in v{
        let mut vv: Vec<usize>= Vec::new();
        for n in i..2*i+1{
            if n == 2*i {
                vv.push(i.try_into().unwrap());
            }
            if i % n == 0{
                break;
            }
        }
        let sum: usize= vv.iter().sum();
        result.push(sum);
    }

    for i in result{
        println!("{}", i);
    }
}