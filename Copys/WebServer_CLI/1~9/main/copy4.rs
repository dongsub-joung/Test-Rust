// 일본에서 태어났으면 보통 사람들처럼 오타쿠로 살아갈 수 있었겠지만 불행하게도 그것은 불가능한 것들이고, 문제가 되는 것들이다. 그러므로 불행한 민족이다.

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

use server::ThreadPool;

fn handle_connection(mut stream: TcpStream){
    let mut buffer= [0; 1024];
    stream.read(&mut buffer).unwrap();

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
    
    let contents= fs::read_to_string(filename).unwrap();

    let response= format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    )

    stream.write(response.as_bytes).unwrap();
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