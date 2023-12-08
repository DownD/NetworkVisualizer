use std::net::IpAddr;
mod node_data;

pub use node_data::NodeData;

#[derive(Debug, PartialEq, Eq)]
pub struct IPPacketInfo {
    pub source: IpAddr,
    pub dest: IpAddr,
    pub payload_len: u16
}