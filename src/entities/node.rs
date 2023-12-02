use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};
use std::borrow::Borrow;
use std::net::IpAddr;
use crate::math::{Point, convert_bytes_str};
use crate::data::IPPacketInfo;
use crate::data::NodeData;
use crate::entities::Entity;
use macroquad::prelude::*;

const DEFAULT_RADIUS: f32 = 10.0;

pub struct Node{
    data: NodeData,
    pos: Point,
    radius: f32,
}

impl Node{
    pub fn new(ip: IpAddr, pos: &Point) -> Node{
        Node{
            data: NodeData::new(ip),
            pos: *pos,
            radius: DEFAULT_RADIUS,
        }
    }
    pub fn add_packet(&mut self, packet: &IPPacketInfo){
        self.data.add_packet(packet);
    }
}

impl Entity for Node {

    fn draw_tooltip(&self, ctx: &egui::Context){
        egui::show_tooltip_at_pointer(ctx, egui::Id::new("my_tooltip"), |ui| {
                ui.label(format!("IP: {:?}", self.data.ip));
                ui.label(format!("Number of packets received: {:?}", self.data.stats.packet_count_recv));
                ui.label(format!("Data received: {}", convert_bytes_str(self.data.stats.packet_bytes_recv)));
                ui.label(format!("Number of packets sent: {:?}", self.data.stats.packet_count_sent));
                ui.label(format!("Data sent: {}", convert_bytes_str(self.data.stats.packet_bytes_sent)));
        });
    }

    fn move_to(&mut self, point: &Point){
        self.pos = *point;
    }

    fn update(&mut self) {
        
    }

    fn get_position(&self) -> &Point{
        &self.pos
    }

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

    fn draw_debug(&self){

    }

}

impl PartialEq for Node{
    fn eq(&self, other: &Self) -> bool {
        &self.data.ip == &other.data.ip
    }
}

impl PartialEq<IpAddr> for Node{
    fn eq(&self, other: &IpAddr) -> bool {
        &self.data.ip == other
    }
}

impl Eq for Node{}

impl Hash for Node{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.ip.hash(state);
    }
}

impl Borrow<IpAddr> for Node {
    fn borrow(&self) -> &IpAddr {
        &self.data.ip
    }
}