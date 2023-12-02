use macroquad::prelude::*;
use macroquad::rand;
use crate::packet_manager::{PacketManager, IPPacketInfo};
use std::collections::HashMap;
use std::hash;
use std::net::IpAddr;
use std::sync::mpsc::Receiver;

const SPEED_PARTICLES: f32 = 1.0;
const DELETE_DISTANCE: f32 = 3.0;

struct NodeRenderData{
    ip: IpAddr,
    x: f32,
    y: f32
}

impl NodeRenderData{
    fn draw(&self){
        draw_circle(self.x, self.y, 10.0, YELLOW);
    }
}

struct PacketRenderData{
    source: IpAddr,
    dest: IpAddr,
    x: f32,
    y: f32
}

impl PacketRenderData{
    fn draw(&self){
        draw_circle(self.x, self.y, 2.0, RED);
    }

    fn distance(&self, x: f32, y: f32) -> f32{
        let x_diff = self.x - x;
        let y_diff = self.y - y;

        return (x_diff.powf(2.0) + y_diff.powf(2.0)).sqrt();
    }
}


pub struct UI {
    packet_manager: PacketManager,
    channel_recv: Receiver<IPPacketInfo>,

    node_position_map: HashMap<IpAddr,NodeRenderData>,
    packet_list: Vec<PacketRenderData>,
}

impl UI {
    pub fn new(channel_recv: Receiver<IPPacketInfo>) -> Self {
        rand::srand(macroquad::miniquad::date::now() as _);
        UI{
            packet_manager:PacketManager::new(),
            channel_recv: channel_recv,
            node_position_map: HashMap::new(),
            packet_list: Vec::new()
        }
    }

    fn add_node_if_not_exists<'a>(hash_map: &'a mut HashMap<IpAddr,NodeRenderData>, ip: &IpAddr) -> &'a NodeRenderData{
        return hash_map.entry(*ip).or_insert(NodeRenderData{
            ip: *ip,
            x: rand::gen_range(0.0,screen_width()),
            y: rand::gen_range(0.0,screen_height())
        });
    }

    fn add_packet_if_not_exists(vec_packet: &mut Vec<PacketRenderData>, packet_info: &IPPacketInfo, src_node: &NodeRenderData){
        vec_packet.push(PacketRenderData{
            source: packet_info.source,
            dest: packet_info.dest,
            x: src_node.x,
            y: src_node.y
        });
    }

    fn listen_packets(&mut self){
        self.channel_recv.try_iter().for_each(|packet| {
            println!("Packet received");
            UI::add_node_if_not_exists(&mut self.node_position_map,&packet.dest);
            let src_node = UI::add_node_if_not_exists(&mut self.node_position_map ,&packet.source);
            UI::add_packet_if_not_exists(&mut self.packet_list, &packet, &src_node);
            self.packet_manager.add_ip_packet(packet);
        }); 
    }

    fn get_unit_vector(x: f32, y: f32) -> (f32,f32){
        let magnitude = (x.powf(2.0) + y.powf(2.0)).sqrt();
        return (x/magnitude, y/magnitude);
    }

    fn update_particle_movement(node_position: &HashMap<IpAddr,NodeRenderData>, packet_list: &mut Vec<PacketRenderData>){
        for packet in packet_list.iter_mut(){
            let dest_node = node_position.get(&packet.dest).unwrap();
            let src_node = node_position.get(&packet.source).unwrap();

            let x_diff = dest_node.x - packet.x;
            let y_diff = dest_node.y - packet.y;

            let (x_unit, y_unit) = UI::get_unit_vector(x_diff, y_diff);

            let x_speed = x_unit * SPEED_PARTICLES;
            let y_speed = y_unit * SPEED_PARTICLES;
            
            packet.x += x_speed;
            packet.y += y_speed;
        }

        packet_list.retain(|packet| {
            let dest_node = node_position.get(&packet.dest).unwrap();
            let distance = packet.distance(dest_node.x, dest_node.y);
            let result = distance > DELETE_DISTANCE;
            return result;
        });
    }

    pub async fn run(&mut self) {
        clear_background(BLACK);

        for node in self.node_position_map.values(){
            node.draw();
        }

        for packet in self.packet_list.iter(){
            packet.draw();
        }

        UI::update_particle_movement(&self.node_position_map, &mut self.packet_list);

        //draw_text("text", x, y, font_size, color)
        draw_text(&format!("Valid packets: {}", self.packet_manager.get_valid_packet_count()), 10.0, 10.0, 20.0, WHITE);
        
        self.listen_packets();
        next_frame().await
    }
}