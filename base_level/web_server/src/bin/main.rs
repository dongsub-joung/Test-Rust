use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

extern crate web_server;
use web_server::ThreadPool;

fn main() {
    let listener= TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool= ThreadPool::new();
    
    for stream in listener.incoming(){
        let stream= stream.unwrap();

        pool.execute(||{
            handle_connnection(stream);
        });
    }
}

fn handle_connnection(mut stream: TcpStream){
    let mut buffer= [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let get= b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html");
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html");
    };

    let mut file= File::open(filename).unwrap();
    let mut contents= String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}