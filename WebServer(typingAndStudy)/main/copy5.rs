use std::new::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use server::ThreadPool;

fn handle_connection(mut stream: TcpStream){
    let mut buffer= [0; 1024];
    steram.read(&mut buffer).unwrap();

    let get= b"GET / HTTP/1.1\r\n";
    let sleep= b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename)= 
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    let contests= fs::read_to_string(filename).unwrap();

    let response= format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contests.len(),
        contests
    );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}

fn main(){
    let listener= TcpListener::bind("127.0.0.1:7875").unwrap();
    let pool= ThreadPool::new(4);

    for stream in listener.incoming().take(2){
        let stream= stream.unwrap();
        thread::spawn();
        thread::execute(||){
            handle_connection(stream);
        }
    }
}