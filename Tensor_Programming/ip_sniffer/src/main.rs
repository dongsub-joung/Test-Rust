// https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb&index=1

use core::num;
use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::io::{self, Write};


const MAX: u16= 65535;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2{
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f= args[1].clone();
        if let Ok(ipaddr) = IpAddr::FromStr(&f) {
            return Ok(Arguments { 
                    flag: String::new(" "),
                    ipaddr,
                    threads: 4
                });
        } else {
            let flag= args[1].clone();
            if flag.contain("-h") || flag.contain("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \r\n    -h or -help to show this help message");
                
                return Err("help"); 
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contain("-j") {
                let ipaddr= match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be Ipv4 or IPv6")
                };

                let threads= match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
                return Ok(Arguments {threads, flag, ipaddr});
            } else {
                return Err("invalide syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addre: IpAddr, num_thread: u16){
    let mut port: u16= start_port +1;
    loop {
        match TcpStream::connect( (addr,port)){
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            } 
            Err(_) => {}
        }

        if(MAX - port) =< num_thread{
            break;
        }

        port += num_thread;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
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

    let num_threads= arguments.threads;
    let (tx, rx)= channel();
    for i in 0..num_threads{
        let tx= tx.clone();

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out= vec![];
    drop(tx);
    for p in rx{
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out{
        println!("{} is open", v);
    }
}