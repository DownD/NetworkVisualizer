use macroquad::prelude::*;
use macroquad::rand;
use std::time::Instant;
use crate::packet_manager::{PacketManager, IPPacketInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::mpsc::Receiver;
use egui::containers::popup;
use crate::math::{Point,Vector};

const DELETE_DISTANCE: f32 = 10.0;
const INITIAL_PACKET_SIZE: usize = 100000;

const MAX_PACKETS: u32 = 150;
const ANGLE_LAUNCH: f32 = 0.1;
const VELOCITY_LAUNCH: f32 = 2.0;

struct NodeRenderData{
    ip: IpAddr,
    pos: Point,
    radius: f32,
}

impl NodeRenderData{
    fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    fn is_point_inside(&self, point: &Point) -> bool{
        let distance = self.pos.distance(point);
        if distance < self.radius{
            return true;
        }
        return false;
    }
}

struct PacketRenderData{
    src: Point,
    pos: Point,
    dest: Point,
    velocity: Vector,
}

impl PacketRenderData{
    fn new(src_point: &Point, dst_point: &Point, start_velocity: f32, start_angle: f32) -> Self{
        let unit_vector = src_point.get_unit_vector(dst_point);
        let rotated_vector = unit_vector.rotate(rand::gen_range(-start_angle,start_angle));
        let force_multiplier: f32 = start_velocity;

        PacketRenderData{
            src: Point{x:src_point.x,y:src_point.y},
            pos: Point{x:src_point.x,y:src_point.y},
            dest: Point{x:dst_point.x,y:dst_point.y},
            velocity: &rotated_vector * force_multiplier
        }
    }

    fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, 1.5, RED);
    }

    fn draw_debug(&self){
        draw_line(self.pos.x, self.pos.y, self.dest.x, self.dest.y, 1.0, BLUE);
        draw_line(self.pos.x, self.pos.y, self.pos.x+(self.velocity.x*10.0), self.pos.y+(self.velocity.y*10.0), 3.0, RED);
        draw_line(self.src.x, self.src.y, self.pos.x, self.pos.y, 1.0, GREEN);
    }
    fn update_fixed(&mut self){
        let vec_to_dest_unit = self.pos.get_unit_vector(&self.dest);
        let vec_velocity_unit = self.velocity.get_unit_vector();
        let dist_to_dest = self.pos.distance(&self.dest);
        let angle = vec_to_dest_unit.angle(&vec_velocity_unit);
        
        self.velocity = self.velocity.rotate(angle/dist_to_dest);

        self.pos += &self.velocity;
    }
}


pub struct UI {
    packet_manager: PacketManager,
    channel_recv: Receiver<IPPacketInfo>,

    node_position_map: HashMap<IpAddr,NodeRenderData>,
    packet_list: Vec<PacketRenderData>,

    //Buttons
    update: bool,
    draw_debug: bool,
    max_packets: u32,
    
    angle_launch: f32,
    speed_launch: f32,
}

impl UI {
    pub fn new(channel_recv: Receiver<IPPacketInfo>) -> Self {
        rand::srand(macroquad::miniquad::date::now() as _);
        let mut packet_list: Vec<PacketRenderData> = Vec::new();
        packet_list.reserve(INITIAL_PACKET_SIZE);
        
        UI{
            packet_manager:PacketManager::new(),
            channel_recv: channel_recv,
            node_position_map: HashMap::new(),
            packet_list: packet_list,
            update: true,
            draw_debug: false,
            max_packets: MAX_PACKETS,
            angle_launch: ANGLE_LAUNCH,
            speed_launch: VELOCITY_LAUNCH
        }
    }

    fn add_packet_nodes<'a>(hash_map: &'a mut HashMap<IpAddr,NodeRenderData>, ip_packet_info: &'a IPPacketInfo) -> (&'a NodeRenderData,&'a NodeRenderData){
        hash_map.entry(ip_packet_info.source).or_insert(NodeRenderData{
            ip: ip_packet_info.source,
            pos: Point{
                x:rand::gen_range(0.0,screen_width()),
                y: rand::gen_range(0.0,screen_height())
            },
            radius: 10.0
        });

        hash_map.entry(ip_packet_info.dest).or_insert(NodeRenderData{
            ip: ip_packet_info.dest,
            pos: Point{
                x:rand::gen_range(0.0,screen_width()),
                y: rand::gen_range(0.0,screen_height())
            },
            radius: 10.0
        });

        let src = hash_map.get(&ip_packet_info.source).unwrap();
        let dst = hash_map.get(&ip_packet_info.dest).unwrap();

        return (src, dst);
    }

    fn listen_packets(&mut self){
        self.channel_recv.try_iter().for_each(|packet| {
            if self.update && self.packet_list.len() < self.max_packets as usize{
                let (src_node,dst_node)= UI::add_packet_nodes(&mut self.node_position_map,&packet);
                self.packet_list.push(PacketRenderData::new(&src_node.pos, &dst_node.pos, self.speed_launch, self.angle_launch));
                self.packet_manager.add_ip_packet(packet);
            }
        }); 
    }

    fn update_particle_movement(packet_list: &mut Vec<PacketRenderData>){
        for packet in packet_list.iter_mut(){
            packet.update_fixed();
        }

        packet_list.retain(|packet| {   
            if packet.pos.distance(&packet.dest) < DELETE_DISTANCE{
                return false;
            }         
            if packet.pos.get_unit_vector(&packet.dest).dot(&packet.src.get_unit_vector(&packet.dest)) < 0.0{
                return false;
            }
            return true;
        });
    }

    fn draw_hud(&mut self, start_timestamp: Instant){
        draw_text(&format!("Valid packets: {}", self.packet_manager.get_valid_packet_count()), 10.0, 10.0, 20.0, WHITE);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
            .show(egui_ctx, |ui| {
                ui.add(egui::Slider::new(&mut self.max_packets, 0..=100000).logarithmic(true).text("Max Packets"));
                ui.label(format!("Number of packets: {:?}", self.packet_list.len()));
                ui.checkbox(&mut self.update, "Start/Stop particle update");
                // Checkbox
                ui.checkbox(&mut self.draw_debug, "Draw debug");
                ui.add(egui::Slider::new(&mut self.angle_launch, 0.0..=1.57).text("Angle launch"));
                ui.add(egui::Slider::new(&mut self.speed_launch, 0.0..=2.0).text("Speed launch"));
            });
            
        });
        egui_macroquad::draw();
        draw_text(&format!("FPS: {:?}", 1.0/start_timestamp.elapsed().as_secs_f64()), screen_width()-130.0, 30.0, 20.0, WHITE);
    }

    pub async fn run(&mut self) {
        let start_timestamp = Instant::now();
        clear_background(BLACK);
        self.listen_packets();

        for node in self.node_position_map.values(){
            node.draw();
        }

        if self.update{
            UI::update_particle_movement(&mut self.packet_list);
        }
        for packet in self.packet_list.iter(){
            if self.draw_debug{
                packet.draw_debug();
            }else{
                packet.draw();
            }
        }

        self.draw_hud(start_timestamp);
        next_frame().await
    }
}