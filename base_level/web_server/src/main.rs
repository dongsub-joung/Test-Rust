use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    let listener= TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream= stream.unwrap();

        handle_connnection(stream);
    }
}

fn handle_connnection(mut stream: TcpStream){
    let mut buffer= [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let get= b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        let mut file= File::open("index.html").unwrap();
        let mut contents= String::new();
        file.read_to_string(&mut contents).unwrap();

        let responese= format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    
        stream.write(responese.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line= "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file= File::open("404.html").unwrap();
        let mut contents= String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}