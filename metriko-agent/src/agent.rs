
use std::{io::Write, collections::VecDeque};

use crate::errors::AgentErrors;
#[allow(unused)]
pub fn send_data_to_server(collectet_data: &[u8]) -> Result<(), AgentErrors>
{
   // let bytes_to_send = metriko_common::encode(collectet_data);
    println!("encoding {} bytes of data ", collectet_data.len());
    let mut tcp_stream = std::net::TcpStream::connect(metriko_common::METRIKO_PORT)
    .map_err(|_| AgentErrors::UnableToConnectErr)?;   
    tcp_stream
    .write_all(collectet_data)
    .map_err(|_| AgentErrors::UnableToConnectErr)?;
    Ok(())
    /*
        We're using map_err to translate the internal errors into our own error format.
     */
}

pub fn send_queque_to_server(queque : &mut VecDeque<Vec<u8>>) -> Result<(),AgentErrors> 
{
   let mut tcp_stream = std::net::TcpStream::connect(metriko_common::METRIKO_PORT)
   .map_err(|_| AgentErrors::UnableToConnectErr)?;
   while let Some(data) = queque.pop_front()  {
       if tcp_stream.write_all(&data).is_err() {
         
         queque.push_front(data);
         return  Err(AgentErrors::UnableToConnectErr);
       }
   }
   Ok(())
} 