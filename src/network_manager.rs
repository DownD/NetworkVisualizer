
use crate::data::IPPacketInfo;
use etherparse::{PacketHeaders,IpHeader};
use pcap::{Capture, Device, Active};
use std::net::IpAddr;
use std::sync::mpsc::Sender;

pub struct NetworkManager {
    sender: Sender<IPPacketInfo>,
    cap: Option<Capture<Active>>,
}

impl NetworkManager{
    pub fn new(channel: Sender<IPPacketInfo>) -> NetworkManager{
        NetworkManager{
            sender: channel,
            cap: None,
        }
    }

    pub fn set_device(&mut self, device: Device){
        self.cap = Some(Capture::from_device(device).unwrap().immediate_mode(true).open().unwrap().setnonblock().unwrap());
    }

    pub fn get_network_devices() -> Vec<Device>{
        return Device::list().unwrap();
    }

    fn convert_to_ip_packet_info(network_header: &IpHeader) -> IPPacketInfo{
        match network_header {
            IpHeader::Version4(ipv4header, _) => {
                IPPacketInfo{
                    source: IpAddr::V4(ipv4header.source.into()),
                    dest: IpAddr::V4(ipv4header.destination.into()),
                    payload_len: ipv4header.payload_len
                }
            }
            IpHeader::Version6(ipv6header, _) => {
                IPPacketInfo{
                    source: IpAddr::V6(ipv6header.source.into()),
                    dest: IpAddr::V6(ipv6header.destination.into()),
                    payload_len: ipv6header.payload_length
                }
            }
        }
    }
        

    pub fn listen_packets(&mut self) -> Option<()>{
        if let Some(capture) = &mut self.cap{
            while let Some(packet) = capture.next_packet().ok(){
                let ip_packet = PacketHeaders::from_ethernet_slice(&packet).ok()?;
                let ip_header = ip_packet.ip?;
                let packet = NetworkManager::convert_to_ip_packet_info(&ip_header);
                self.sender.send(packet).unwrap();
            }
        }
        return Some(());
    }

}