use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize ; 
use serde::Deserialize;
pub const METRIKO_PORT : &str = "0.0.0.0:9094";
const MAGIC_NUMBER : u16 = 9510 ;
const VERSION_NUMBER : u16 = 1 ; 
#[derive(Debug,Serialize,Deserialize,Clone,PartialEq)]
pub enum MetrikoApi {
 SubmitData  {
    collector_id : u128 , 
    total_memory : u64 , 
    used_memory : u64 , 
    avg_cpu_usage : f32 , 
 }
}
pub fn encode(command : &MetrikoApi) -> Vec<u8> {
    let json = serde_json::to_string(&command).unwrap() ; 
    let json_bytes = json.as_bytes(); 
    let crc = crc32fast::hash(json_bytes); 
    let payload_size = json_bytes.len() as u32 ; 
    let timestamp = unix_now() ; 
    // encode into bytes 
    let mut res = Vec::with_capacity(140);
    res.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    res.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    res.extend_from_slice(&timestamp.to_be_bytes());
    res.extend_from_slice(&payload_size.to_be_bytes());
    res.extend_from_slice(json_bytes);
    res.extend_from_slice(&crc.to_be_bytes());
    res 
}
pub fn decode(bytes : &[u8]) -> (u32,MetrikoApi) {
    let magic_num = u16::from_be_bytes([bytes[0],bytes[1]]);
    let version = u16::from_be_bytes([bytes[2],bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4],bytes[5],bytes[6],bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8],bytes[9],bytes[10],bytes[11]]);
    let payload = &bytes[12..12+ payload_size as usize];
    let crc = u32::from_be_bytes([
        bytes[12 + payload_size as usize],
        bytes[13 + payload_size as usize],
        bytes[14 + payload_size as usize],
        bytes[15 + payload_size as usize],
        
    ]);
    assert_eq!(magic_num,MAGIC_NUMBER);
    assert_eq!(version,VERSION_NUMBER);
    let cumputed_crc = crc32fast::hash(payload);
    assert_eq!(crc,cumputed_crc);
    (timestamp,serde_json::from_slice(payload).unwrap())
}
fn unix_now() -> u32 {
    let start = SystemTime::now() ; 
    let since = start
    .duration_since(UNIX_EPOCH)
    .expect("time went backwards");
    since.as_secs() as u32 
}
#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn test_enc_dec() {
        let cmd = MetrikoApi::SubmitData { collector_id: 144758, total_memory: 100, used_memory: 50, avg_cpu_usage: 0.1 };
        let encoded = encode(&cmd);
        let (timestamp,decoded) = decode(&encoded);
        assert_eq!(decoded,cmd);
        assert!(timestamp > 0);
    }
}