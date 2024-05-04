use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind a TCP listener to a local address.
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // Accept incoming connections in a loop.
    loop {
        // Asynchronously wait for a new connection.
        let (mut socket, _) = listener.accept().await?;

        // Spawn a task to handle the connection.
        tokio::spawn(async move {
            // Read data from the socket.
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            println!("Read {} bytes from the socket.", n);

            // Write data to the socket.
            socket.write_all(b"Hello, world!").await.unwrap();
        });
    }
}
