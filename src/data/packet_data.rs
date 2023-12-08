use std::net::IpAddr;
use crate::data::IPPacketInfo;

pub struct PacketData{
    pub source: IpAddr,
    pub dest: IpAddr,
}

impl PacketData{
    pub fn new(packet: &IPPacketInfo) -> PacketData{
        PacketData{
            source: packet.source,
            dest: packet.dest,
        }
    }
}