use std::collections::HashMap;
use std::vec::Vec;
use std::net::IpAddr;
use macroquad::prelude::*;
use crate::math::Point;
use crate::data::IPPacketInfo;
use crate::engine::{EngineSettings, PhysicsEngine, GraphicsEngine};
use crate::entities::{PacketGraphics, Entity, Node};

pub struct Engine {
    settings: EngineSettings,
    node_position_map: HashMap<IpAddr,Node>,
    packet_position_map: Vec<PacketGraphics>
}


impl Engine {
    pub fn new() -> Engine{
        rand::srand(macroquad::miniquad::date::now() as _);
        let settings = EngineSettings {
            update: true,
            draw_debug: false,
            max_packets: 50000,
            
            angle_launch: 0.2,
            speed_launch: 1.5,
            delete_distance: 8.0,
        };

        let mut hashmap: HashMap<IpAddr,Node> = HashMap::new();
        hashmap.reserve(500);

        let mut packet_vector: Vec<PacketGraphics> = Vec::new();
        packet_vector.reserve(settings.max_packets as usize);

        Engine{
            settings: settings,
            node_position_map: hashmap,
            packet_position_map: packet_vector
        }
    }

    pub fn get_visible_packet_count(&self) -> usize{
        return self.packet_position_map.len();
    }

    pub fn get_settings(&mut self) -> &mut EngineSettings {
        return &mut self.settings;
    }

    fn get_screen_random_position() -> Point{
        Point{
            x:rand::gen_range(0.0,screen_width()),
            y: rand::gen_range(0.0,screen_height())
        }
    }
    
}


impl PhysicsEngine for Engine {
    fn get_settings(&mut self) -> &mut EngineSettings {
        return &mut self.settings;
    }

    fn add_packet(&mut self, packet: &IPPacketInfo) {

        // Insert source
        let source_node = self.node_position_map.entry(packet.source).or_insert(Node::new(packet.source,&Engine::get_screen_random_position()));
        source_node.add_packet(packet);
        let source_pos = source_node.get_position().clone();

        // Insert dest
        let dest_node = self.node_position_map.entry(packet.dest).or_insert(Node::new(packet.dest,&Engine::get_screen_random_position()));
        dest_node.add_packet(packet);
        let dest_pos = dest_node.get_position();

        // Check max packets
        if self.settings.max_packets < self.packet_position_map.len() as u32{
            return;
        }

        // Insert packet
        self.packet_position_map.push(PacketGraphics::new(&source_pos,dest_pos,self.settings.speed_launch,self.settings.angle_launch));
    }

    fn update(&mut self) {
        if !self.settings.update{
            return;
        }

        for packet in &mut self.packet_position_map{
            packet.update();
        }

        self.packet_position_map.retain(|packet| {   
            if packet.get_position().distance(&packet.get_destination()) < self.settings.delete_distance{
                return false;
            }         
            if packet.get_position().get_unit_vector(packet.get_destination()).dot(&packet.get_source().get_unit_vector(packet.get_destination())) < 0.0{
                return false;
            }
            return true;
        });
    }
}


impl GraphicsEngine for Engine {
    fn get_settings(&mut self) -> &mut EngineSettings {
        return &mut self.settings;
    }

    fn draw(&self) {
        
        self.node_position_map.iter().for_each(|(_,node)| node.draw());
        self.packet_position_map.iter().for_each(|packet| packet.draw());

        if !self.settings.draw_debug{
            return;
        }

        self.node_position_map.iter().for_each(|(_,node)| node.draw_debug());
        self.packet_position_map.iter().for_each(|packet| packet.draw_debug());
    }
}