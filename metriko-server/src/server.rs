use std::net::SocketAddr;

use metriko_common::METRIKO_PORT;
use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};

pub async fn collect_data() -> anyhow::Result<()> {
    let tcp_listenet = TcpListener::bind(METRIKO_PORT).await?;
    loop {
            let (socket , addr ) = tcp_listenet.accept().await?;
            tokio::spawn(new_connection(socket, addr));
    }
}
async fn new_connection(mut stream : TcpStream , addr : SocketAddr) {
    println!("new connection from addr : {addr:?}");
    let mut buf = vec![0u8;1024];
    let n =  stream
    .read(&mut buf)
    .await 
    .expect("faile to read from socket"); 
    if n == 0 {
        return  ;
    }
    println!("received {n} bytes");
    let received_data = metriko_common::decode(&buf[0..n]);
    println!("received data : {received_data:?}")

}