// https://www.youtube.com/watch?v=BHxmWTVFWxQ

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn handle_connection(mut stream: TcpStream){
    let mut buffer: [i32; 1024]= [0; 1024];

    stream.read(&mut buffer).unwrap();
    prinln!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    );
}

fn main(){
    let listener: TcpListener=
        TcpListener::bind("127.0.0.1:7875").unwrap();

    for stream: Result<TcpStream, Error> in listener.incoming(){
        let stream: TcpStream= stream.unwrap();

        handle_connection(stream);
    }
}
