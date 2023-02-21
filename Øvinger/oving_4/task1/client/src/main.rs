use tokio::io::{stdin, stdout};
use tokio::net::UdpSocket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Client");
    println!("Binding socket");
    let socket = UdpSocket::bind("0.0.0.0:7878").await?;

    socket.connect("127.0.0.1:8080").await?;

    let mut stdin = stdin();

    println!("Ready to receive math pieces");
    loop {
        let mut stdin_buf = [0; 512];
        let mut receiver_buf = [0; 512];

        tokio::select! {
            res = stdin.read(&mut stdin_buf) => {
                if let Ok(amount) = res {
                        socket.send(&stdin_buf[0..amount]).await.map_err(|_| "failed to write to the socket").unwrap();
                } else {
                    res.unwrap();
                }
            }
            res = socket.recv(&mut receiver_buf) => {
                if let Ok(amount) = res {
                    stdout().write(&receiver_buf[0..amount]).await.map_err(|_| "failed to write to stdout").unwrap();
                } else {
                    res.unwrap();
                }
            }
        }
    }
}
