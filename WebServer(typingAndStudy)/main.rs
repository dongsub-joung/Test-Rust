use std::net::TcpListener;

fn main(){
    let listener: TcpListener=
        TcpListener::bind("127.0.0.1:7875").unwrap();

    for stream: Result<TcpStream, Error> in listener.incoming(){
        let stream: TcpStream= stream.unwrap();

        prinln!("Connection established!");
    }
}