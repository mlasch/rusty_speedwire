use std::error::Error;

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Start UDP server");
    let addr = String::from("localhost:8000");
    let mut buf: [u8; 1024] = [0; 1024];
    let sock = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", sock.local_addr()?);

    loop {
        let data = Some(sock.recv_from(&mut buf).await?);
        println!("Received: {:?} {:?}", data, buf);
    }
}
