use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("making socket");
    let socket = UdpSocket::bind("0.0.0.0:7878").await?;
    println!("defining buffer");
    let mut buffer = [0; 1024];

    loop {
        println!("Receiving bytes");
        let (len, addr) = socket.recv_from(&mut buffer).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        let received = std::str::from_utf8(&buffer);
        println!("Received {:?}", received);

        println!("Writing to buffer");
        let buffer = &mut buffer[..len];
        buffer.reverse();

        println!("Received: {}", len);

        socket.send_to(buffer, addr).await?;
        println!("{:?} bytes sent", len);
    }
}
