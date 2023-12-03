use macroquad::prelude::*;
use macroquad::rand;
use crate::packet_manager::{PacketManager, IPPacketInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::mpsc::Receiver;
use crate::math::{Point, get_circle_point};

const SPEED_PARTICLES: f32 = 0.01;
const DELETE_DISTANCE: f32 = 3.0;

struct NodeRenderData{
    ip: IpAddr,
    pos: Point
}

impl NodeRenderData{
    fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, 10.0, YELLOW);
    }
}

struct PacketRenderData{
    src: Point,
    pos: Point,
    p_bazier: Point,
    t_bazier: f32,
    dest_ip: IpAddr
}

impl PacketRenderData{

    fn new(packet_info: &IPPacketInfo, src_point: &Point, dst_point: &Point) -> Self{
        let m_p = src_point.get_middle_point(dst_point);
        let dist = src_point.distance(dst_point);
        let p_bazier = get_circle_point(&m_p, rand::gen_range(0.0,6.0), rand::gen_range(0.1,dist));

        PacketRenderData{
            src: Point{x:src_point.x,y:src_point.y},
            pos: Point{x:src_point.x,y:src_point.y},
            p_bazier: p_bazier,
            t_bazier: 0.0,
            dest_ip: packet_info.dest
        }

    }
    fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, 2.0, RED);
    }

    fn update_bazier_position(&mut self, dest: &Point){
        self.t_bazier += SPEED_PARTICLES;
        self.pos.set_bazier_next_point(&self.src, &self.p_bazier, &dest, self.t_bazier);
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

    fn add_packet_nodes<'a>(hash_map: &'a mut HashMap<IpAddr,NodeRenderData>, ip_packet_info: &'a IPPacketInfo) -> (&'a NodeRenderData,&'a NodeRenderData){
        hash_map.entry(ip_packet_info.source).or_insert(NodeRenderData{
            ip: ip_packet_info.source,
            pos: Point{
                x:rand::gen_range(0.0,screen_width()),
                y: rand::gen_range(0.0,screen_height())
            }
        });

        hash_map.entry(ip_packet_info.dest).or_insert(NodeRenderData{
            ip: ip_packet_info.dest,
            pos: Point{
                x:rand::gen_range(0.0,screen_width()),
                y: rand::gen_range(0.0,screen_height())
            }
        });

        let src = hash_map.get(&ip_packet_info.source).unwrap();
        let dst = hash_map.get(&ip_packet_info.dest).unwrap();

        return (src, dst);
    }

    fn add_packet(vec_packet: &mut Vec<PacketRenderData>, packet_info: &IPPacketInfo, src_node: &NodeRenderData, dst_node: &NodeRenderData){
        vec_packet.push(PacketRenderData::new(packet_info, &src_node.pos, &dst_node.pos));
    }

    fn listen_packets(&mut self){
        self.channel_recv.try_iter().for_each(|packet| {
            println!("Packet received: from {} to {}", packet.source, packet.dest);
            let (src_node,dst_node)= UI::add_packet_nodes(&mut self.node_position_map,&packet);
            UI::add_packet(&mut self.packet_list, &packet, &src_node, &dst_node);
            self.packet_manager.add_ip_packet(packet);
        }); 
    }

    fn update_particle_movement(node_position: &HashMap<IpAddr,NodeRenderData>, packet_list: &mut Vec<PacketRenderData>){
        for packet in packet_list.iter_mut(){
            let dest_node = node_position.get(&packet.dest_ip).unwrap();
            packet.update_bazier_position(&dest_node.pos);
            //let dest_node = node_position.get(&packet.dest).unwrap();
            //let src_node = node_position.get(&packet.source).unwrap();
//
            //let x_diff = dest_node.x - packet.x;
            //let y_diff = dest_node.y - packet.y;
//
            //let (x_unit, y_unit) = UI::get_unit_vector(x_diff, y_diff);
//
            //let x_speed = x_unit * SPEED_PARTICLES;
            //let y_speed = y_unit * SPEED_PARTICLES;
            //
            //packet.x += x_speed;
            //packet.y += y_speed;
        }

        packet_list.retain(|packet| {
            let dest_node = node_position.get(&packet.dest_ip).unwrap();
            let distance = packet.pos.distance(&dest_node.pos);
            let result = distance > DELETE_DISTANCE;
            return result;
        });
    }

    pub async fn run(&mut self) {
        clear_background(BLACK);
        self.listen_packets();

        for node in self.node_position_map.values(){
            node.draw();
        }

        for packet in self.packet_list.iter(){
            packet.draw();
        }

        UI::update_particle_movement(&self.node_position_map, &mut self.packet_list);

        //draw_text("text", x, y, font_size, color)
        draw_text(&format!("Valid packets: {}", self.packet_manager.get_valid_packet_count()), 10.0, 10.0, 20.0, WHITE);
        next_frame().await
    }
}