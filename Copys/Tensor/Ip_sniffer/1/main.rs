// https://github.com/tensor-programming/Rust_Port_Sniffer/blob/master/src/main.rs

use std::env;
use std::io::{self, Write};
use std::net::{ IpAddr, TcpStream };
use std::ops::RangeBounds;
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{ Sender, channel };
use std::thread;

const MAX: u16= 65535;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2{
            return Err("not enought arguments");
        } else if args.len() > 4{
            return Err("too many arguments");
        } 
        let f= args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 });
        } else{
            let flag= args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j top select how many thread you want
                \n\r    -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help"){
                return Err("too many arguments");
            } else if flag.contains("-j"){
                let ipaddr= match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be Ipv4 or Ipv6")
                };
                let threads= match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };

                return Ok(Arguments{ threads, flag, ipaddr} );
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port= start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads{
            break;
        }
        port += num_threads;
    }
}

fn main(){
    let args: Vec<String>= env::args().collect();
    let program= args[0].clone();
    let arguments= Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads= argumnets.threads;
    let (tx, rx)= channel();
    for i in 0..num_thread{
        let tx= tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out= Vec![];
    drop(tx);
    for p in rx{
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        prinlnt!("{} is open" , v);
    }

}