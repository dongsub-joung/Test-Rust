// https://www.youtube.com/watch?v=BHxmWTVFWxQ

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn handle_connection(mut stream: TcpStream){
    let mut buffer: [i32; 1024]= [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get: &[u8; 16]= b"GET / HTTP/1.1\r\n";
    
    let (status_line, finlename)= 
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents: String= 
        fs::read_to_string(finlename).unwrap();
    
    let response: String= format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main(){
    let listener: TcpListener=
        TcpListener::bind("127.0.0.1:7875").unwrap();

    for stream: Result<TcpStream, Error> in listener.incoming(){
        let stream: TcpStream= stream.unwrap();

        handle_connection(stream);
    }
}
