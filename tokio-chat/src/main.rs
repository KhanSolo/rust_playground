use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, _) = listener.accept().await.unwrap();

    let (read, mut write) = socket.split();

    let mut reader = BufReader::new(read);
    let mut line = String::new();

    loop {
        let bytes_len = reader.read_line(&mut line).await.unwrap();
        if bytes_len == 0 {
            break;
        }

        write.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}
