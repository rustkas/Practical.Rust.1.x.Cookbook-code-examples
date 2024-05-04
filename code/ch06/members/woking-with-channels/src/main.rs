use futures::channel::mpsc;

#[tokio::main]
async fn main() {
    // Create an MPSC channel.
    let (mut sender, mut receiver) = mpsc::channel(10);

    // Spawn a task to send values to the channel.
    tokio::spawn(async move {
        for i in 0..=10 {
            let _ = sender.try_send(i);
        }
    });

    loop {
        let result: Result<Option<i32>, mpsc::TryRecvError> = receiver.try_next();
        match result {
            Ok(val) => match val {
                Some(value) => {
                    println!("Received value: {value}");
                    continue;
                }
                None => {
                    eprintln!();
                    break;
                }
            },
            Err(err) => eprintln!("{err}"),
        }
    }
}
