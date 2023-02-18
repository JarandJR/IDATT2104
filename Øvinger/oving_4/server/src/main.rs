use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server");
    println!("Binding socket");
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buffer = vec![0; 1024];

    println!("Ready to receive..");
    loop {
        let (len, addr) = socket.recv_from(&mut buffer).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        let received = std::str::from_utf8(&buffer)
            .expect("Failed to convert to utf8")
            .split("\n")
            .nth(0)
            .unwrap();

        println!("Recived  message: {:?}", received);
        let mut answear = match meval::eval_str(received) {
            Ok(a) => a.to_string(),
            Err(e) => e.to_string(),
        };
        answear.push('\n');

        let len = socket.send_to(answear.as_bytes(), addr).await?;
        println!("{:?} bytes sent", len);
    }
}
