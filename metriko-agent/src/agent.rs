
use std::io::Write;

use crate::errors::AgentErrors;

pub fn send_data_to_server(collectet_data: &metriko_common::MetrikoApi) -> Result<(), AgentErrors>{
    let bytes_to_send = metriko_common::encode(collectet_data);
    println!("encoding {} bytes of data ", bytes_to_send.len());
    let mut tcp_stream = std::net::TcpStream::connect(metriko_common::METRIKO_PORT)
    .map_err(|_| AgentErrors::UnableToConnectErr)?;   
    tcp_stream
    .write_all(&bytes_to_send)
    .map_err(|_| AgentErrors::UnableToConnectErr)?;
    Ok(())
    /*
        We're using map_err to translate the internal errors into our own error format.
     */
}
