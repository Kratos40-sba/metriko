use std::io::Write;

pub fn send_data_to_server(collectet_data: metriko_common::MetrikoApi) {
    let bytes_to_send = metriko_common::encode(collectet_data);
    println!("encoding {} bytes of data ", bytes_to_send.len());
    let mut tcp_stream = std::net::TcpStream::connect(metriko_common::METRIKO_PORT).unwrap();
    tcp_stream.write_all(&bytes_to_send).unwrap()
}
