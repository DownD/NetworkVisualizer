use std::net::IpAddr;
use std::collections::HashMap;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};


#[derive(Debug, PartialEq, Eq)]
pub struct IPPacketInfo {
    pub source: IpAddr,
    pub dest: IpAddr,
    pub payload_len: u16
}

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

pub struct Node{
    pub ip: IpAddr,
    pub stats: Statistics,
    pub recv_from: HashMap<IpAddr, Statistics>,
    pub sent_to: HashMap<IpAddr, Statistics>,
}

impl Node{
    fn new(ip: IpAddr) -> Node{
        Node{
            ip: ip,
            stats: Statistics::new(),
            recv_from: HashMap::new(),
            sent_to: HashMap::new(),
        }
    }

    fn add_packet(&mut self, packet: &IPPacketInfo){
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

pub struct PacketManager {
    ip_packets: HashMap<IpAddr,Node>,
    valid_packet_count: u64
}

impl PacketManager {
    pub fn new() -> PacketManager{
        PacketManager{
            ip_packets: HashMap::new(),
            valid_packet_count: 0
        }
    }

    pub fn get_valid_packet_count(&self) -> u64{
        self.valid_packet_count
    }

    pub fn add_ip_packet(&mut self, packet: IPPacketInfo){
        let source_node = self.ip_packets.entry(packet.source).or_insert(Node::new(packet.source));
        source_node.add_packet(&packet);

        let dest_node = self.ip_packets.entry(packet.dest).or_insert(Node::new(packet.dest));
        dest_node.add_packet(&packet);

        self.valid_packet_count += 1;
    }
}