use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};
use std::borrow::Borrow;
use std::net::IpAddr;
use crate::math::Point;
use crate::data::IPPacketInfo;
use crate::data::NodeData;
use crate::entities::Entity;
use crate::entities::node_graphics::NodeGraphics;

const DEFAULT_RADIUS: f32 = 10.0;

pub struct Node{
    data: NodeData,
    graphics: NodeGraphics,
}

impl Node{
    pub fn new(ip: IpAddr, pos: &Point) -> Node{
        Node{
            data: NodeData::new(ip),
            graphics: NodeGraphics::new(*pos, DEFAULT_RADIUS),
        }
    }
    pub fn add_packet(&mut self, packet: &IPPacketInfo){
        self.data.add_packet(packet);
    }

    pub fn get_position(&self) -> &Point{
        self.graphics.get_position()
    }

    pub fn draw(&self){
        self.graphics.draw();
    }

    pub fn draw_debug(&self){
        self.graphics.draw_debug();
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