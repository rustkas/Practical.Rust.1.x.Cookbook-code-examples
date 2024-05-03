// use tokio::prelude::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024]; // Read data from the socket
            let n = socket.read(&mut buf).await.unwrap(); // Write the data back to the socket
            socket.write_all(&buf[0..n]).await.unwrap();
        });
    }
}
