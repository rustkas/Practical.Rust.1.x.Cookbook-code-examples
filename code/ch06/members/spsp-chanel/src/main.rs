use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() {
    // Create an MPSC channel.
    let (sender, mut receiver) = unbounded_channel();

    // Spawn a task to send values to the channel.
    tokio::spawn(async move {
        for i in 0..=10 {
            let _ = sender.send(i);
        }
    });

    
    // Receive values from the channel in the main task.
    while let Some(val) = receiver.recv().await {
        println!("Received value: {}", val);
    }
}
