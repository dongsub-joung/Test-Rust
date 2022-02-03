    use std::net::TcpListener;
    use std::net::TCpStream;
    use std::io::prelude::*;

    use WebServer_CLI::ThreadPool;

    fn handle_connection(mut stream: tcpStream){
        let mut buffer= [0; 1024];
        stream.read(&mut buffer).unwrap();

        let get= b"GET / HTTP/1.1\r\n";
        let sleep= b"GET /sleep HTTP/1.1\r\n";

        let (status_line, filename)= {
            if buffer.starts_with(get){
                ("HTTP/1.1 200 OK", "index.html")
            } else if buffer.starts_with(sleep){
                Thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "index.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            }
        };

        let contents= fs::read_to_string(filename).unwrap();

        let response= format!(
            "{}\r\nContent-Lenght: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
    }

    fn main() {
        
    }
