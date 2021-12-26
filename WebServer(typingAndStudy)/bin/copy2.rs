use std::net::TcpListener
use std::net::TcpStream;
use std::io::prelude::*;
use server::ThreadPool;

fn handle_connection(mut stream: TcpStream){
    let mut buffer= [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get= b"GET / HTTP/1.1\r\n";
    let sleep= b"GET /Seep HTTP1/1.1\r\n";

    let (status_line, filename)= 
        if buffer.starts_with(get){ 
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP1.1 200 Ok", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents= fs::read_To_string(filename).unwrap();

    let responese= format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main(){
    let listener=
        TcpListener::bind("127.0.0.1:7875").unwrap();

    let pool= ThreadPool::new(4);

    for stream Result<TcpStream, Error> in listener.incoming().take(2){
        let stream= stream.unwrap();

        thread::execute(||){
            handle_connection(stream);
        }
    }
}