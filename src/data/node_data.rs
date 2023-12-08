use std::net::IpAddr;
use std::collections::HashMap;
use crate::data::IPPacketInfo;

pub struct Statistics{
    pub packet_count_recv: u32,
    pub packet_count_sent: u32,
    pub packet_bytes_sent: u64,
    pub packet_bytes_recv: u64,
}

impl Statistics{
    fn new() -> Statistics{
        Statistics{
            packet_count_recv: 0,
            packet_count_sent: 0,
            packet_bytes_sent: 0,
            packet_bytes_recv: 0,
        }
    }

    fn add_packet(&mut self, packet: &IPPacketInfo, source_ip: IpAddr){
        if packet.source == source_ip{
            self.packet_count_sent += 1;
            self.packet_bytes_sent += packet.payload_len as u64;
        }
        else if packet.dest == source_ip{
            self.packet_count_recv += 1;
            self.packet_bytes_recv += packet.payload_len as u64;
        }
    }
}

pub struct NodeData{
    pub ip: IpAddr,
    pub stats: Statistics,
    pub recv_from: HashMap<IpAddr, Statistics>,
    pub sent_to: HashMap<IpAddr, Statistics>,
}

impl NodeData{
    pub fn new(ip: IpAddr) -> NodeData{
        NodeData{
            ip: ip,
            stats: Statistics::new(),
            recv_from: HashMap::new(),
            sent_to: HashMap::new(),
        }
    }

    pub fn add_packet(&mut self, packet: &IPPacketInfo){
        if packet.source == self.ip{
            self.stats.packet_count_sent += 1;
            self.stats.packet_bytes_sent += packet.payload_len as u64;
            let stats = self.sent_to.entry(packet.dest).or_insert(Statistics::new());
            stats.add_packet(&packet, self.ip);
        }
        else if packet.dest == self.ip {
            self.stats.packet_count_recv += 1;
            self.stats.packet_bytes_recv += packet.payload_len as u64;
            let stats = self.recv_from.entry(packet.source).or_insert(Statistics::new());
            stats.add_packet(&packet, self.ip);
        }else{
            panic!("Packet does not belong to this node");
        }
    }
    
}
