extern crate tokio;

use std::io;
use tokio::net::{TcpListener, TcpStream};

async fn process_socket(socket: TcpStream) -> io::Result<()> {
    loop {
        let mut buf = [0; 4096];
        match socket.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let mut c = String::from_utf8_lossy(&buf[0..n]).into_owned();
                while c.ends_with("\r") || c.ends_with("\n") {
                    c.pop();
                }
                match c.as_str() {
                    "QUIT" => break,
                    "PING" => socket.try_write(b"PONG\n"),
                    _ => socket.try_write(b"Command not available\n"),
                };
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
    Ok(())
}
