
use crate::packet_manager::IPPacketInfo;
use etherparse::{PacketHeaders,IpHeader};
use pcap::{Capture, Device, Active};
use std::net::Ipv4Addr;
use anyhow::{Result,anyhow};
use std::net::IpAddr;
use std::sync::mpsc::Sender;

static NETWORK_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 2, 107);
static NETWORK_DESCPRITION: &str = "Realtek PCIe GbE Family Controller";
pub struct NetworkManager {
    sender: Sender<IPPacketInfo>,
    cap: Capture<Active>,
}

impl NetworkManager{
    pub fn new(channel: Sender<IPPacketInfo>) -> NetworkManager{
        
        //let device = PacketManager::get_network_device_by_ip(NETWORK_IP).expect("No device available matching the search criteria");
        let device = NetworkManager::get_network_device_by_description(NETWORK_DESCPRITION).expect("No device available matching the search criteria");
    
        // get the default Device
        println!("Using device {:?}", device.desc);
    
        let cap = Capture::from_device(device).unwrap().immediate_mode(true).open().unwrap().setnonblock().unwrap();

        NetworkManager{
            sender: channel,
            cap: cap,
        }
    }
    fn print_all_available_devices(){
        for device in Device::list().unwrap() {
            let vec : Vec<IpAddr> = device.addresses.iter().map(|a | a.addr).collect();
            println!("Addresses:{:?} | Description:{:?}", vec, device.desc);
        }
    }

    fn get_network_device_by_description(desc: &str) -> Option<Device>{
        return Device::list().ok()?.into_iter().find(|d| d.desc == Some(desc.to_string()));
    }

    fn get_network_device_by_ip(ip: Ipv4Addr) -> Option<Device>{
        return Device::list().ok()?.into_iter().find(|d| d.addresses.iter().any(|a| a.addr == ip));
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
        

    pub fn listen_packets(&mut self){
        match self.parse_new_packets() {
            Ok(_) => (),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn parse_new_packets(&mut self) -> Result<()>{

        while let Some(packet) = self.cap.next_packet().ok(){
            let ip_packet = PacketHeaders::from_ethernet_slice(&packet)?;
            let ip_header = ip_packet.ip.ok_or_else(|| anyhow!("No IP header found"))?;
            let packet = NetworkManager::convert_to_ip_packet_info(&ip_header);
            self.sender.send(packet).unwrap();
        }
        
        Ok(())
    }

}