use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, _) = listener.accept().await.unwrap();

    let mut buf = [0u8;1024];
    let length = socket.read(&mut buf).await.unwrap();

    // let x: &str = &String::from_utf8_lossy(&buf[..length]);
    // println!("{} {}", length, x);
    socket.write_all(&buf[..length]).await.unwrap();
}