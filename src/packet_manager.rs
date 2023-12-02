use std::net::IpAddr;
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Debug, PartialEq, Eq)]
pub struct IPPacketInfo {
    pub source: IpAddr,
    pub dest: IpAddr,
    pub payload_len: u16,
    pub payload: Box<[u8]>,
}

pub struct Node{
    pub ip: IpAddr,
    pub packets_sent: Vec<Rc<IPPacketInfo>>,
    pub packets_recv: Vec<Rc<IPPacketInfo>>,
}

impl Node{
    fn new(ip: IpAddr) -> Node{
        Node{
            ip: ip,
            packets_sent: Vec::new(),
            packets_recv: Vec::new(),
        }
    }

    fn add_packet(&mut self, packet: Rc<IPPacketInfo>){
        if packet.source == self.ip{
            self.packets_sent.push(packet);
        }
        else if packet.dest == self.ip {
            self.packets_recv.push(packet);
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
        let packet_heap = Rc::new(packet);

        let source_node = self.ip_packets.entry(packet_heap.source).or_insert(Node::new(packet_heap.source));
        source_node.add_packet(packet_heap.clone());
        
        let dest_node = self.ip_packets.entry(packet_heap.dest).or_insert(Node::new(packet_heap.dest));
        dest_node.add_packet(packet_heap);

        self.valid_packet_count += 1;
    }
}