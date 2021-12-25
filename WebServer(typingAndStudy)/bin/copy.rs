use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use server::ThreadPool;

fn handle_connection(mut stream: TcpStream){
    let mut buffer= [0; 1024]
    stream.read(&mut buffer).unwrap();

    let get  = b"GET / HTTP/1.1 \r\n";
    let sleep= b"GET /sleep HTTP/1.1 \r\n";
  
    let(status_line, filename)= 
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents= fs::read_to_string(filename).unwrap();

    let response= format!(
        "{}\r\n Content-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    )

fn main(){
    const HOME= String::from("127.0.0.1:7875");

    let listener=
        TcpListener::bind(&HOME).unwrap();
    let pool= ThreadPool::new(4);
    
    for stream in listener.incoming().take(2){
        let stream= stream.unwrap();
        thread::spawn();
        thread::execute(||){
            handle_connection(stream);
        }
    }
}