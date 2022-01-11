pub fn incoming(&self) -> Incoming<'_>

use std::net::{ TcpListener, TcpStream };

fn handle_connection(stream: TcpStream){
    // ...
}

fn main() -> std::io::Result<()> {
    let listener= 
}