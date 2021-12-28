pub struct TcpListener(_);

// Ex
use std::new::{TcpListener, TcpStream}

fn handle_client(stream: TcpStream) {}

fn main() -> std::io::Result<()> {
    let listener= TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming(){
        handle_client(stream?);
    }

    Ok(())
}

// Implementations
impl TcpListener

pub fn set_nonblocking(&self, nonblocking: bool) -> Result<()>
// On Unix platforms, calling this method corresponds to calling fcntl FIONBIO. On Windows calling this method corresponds to calling ioctlsocket FIONBIO.

// Ex set_nonblocking
use std::io;
use std::net::TcpListener;

let listener= TcpListener::bind("127.0.0.1:7878").unwrap();
listener.set_nonblocking(true).expect("Cannot set non-blocking");
for stream in listener.incoming(){
    match stream{
        Ok(s) => { handle_client(s); }
        Err(ref e) if e.kind() == io::ErrorKind::Would =>{
//             via platform-specific APIs
            wait_for_fd();
            continue;
        }
        Err(e) => panic!("encountered IO error: {}", e),
    }
}

pub fn bind<A: ToSocketAddres>(addr: A) -> Result<TcpListener>
pub fn local_addr(&self) -> Result<SocketAddr>
pub fn try_clone(&self) -> Result<TcpListener>
pub fn accept(&self) -> Result<(TcpStream, SocketAddr)>
pub fn incoming(&self) -> Incoming<'_>
// pub fn into_incoming(self) -> IntoIncoming
pub fn set_ttl(&self, ttl: u32) -> Result<()>
pub fn ttl(&self) -> Result<u32>
pub fn take_error(&self) -> result<Option<Error>>
