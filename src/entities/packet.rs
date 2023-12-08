use std::net::IpAddr;
use crate::math::{Point,Vector};
use crate::entities::Entity;
use crate::data::{PacketData, IPPacketInfo};
use macroquad::prelude::*;

pub struct PacketEntity{
    src: Point,
    pos: Point,
    dest: Point,
    velocity: Vector,
    packet_data: PacketData,
}

impl PacketEntity{
    pub fn new(packet: &IPPacketInfo, src_point: &Point, dst_point: &Point, start_velocity: f32, start_angle: f32) -> Self{
        let unit_vector = src_point.get_unit_vector(dst_point);
        let rotated_vector = unit_vector.rotate(rand::gen_range(-start_angle,start_angle));
        let force_multiplier: f32 = start_velocity;

        PacketData::new(packet);

        PacketEntity{
            src: Point{x:src_point.x,y:src_point.y},
            pos: Point{x:src_point.x,y:src_point.y},
            dest: Point{x:dst_point.x,y:dst_point.y},
            velocity: &rotated_vector * force_multiplier,
            packet_data: PacketData::new(packet),
        }
    }

    pub fn get_ip_source(&self) -> &IpAddr{
        &self.packet_data.source
    }
    pub fn get_ip_dest(&self) -> &IpAddr{
        &self.packet_data.dest
    }
    pub fn get_source(&self) -> &Point{
        &self.src
    }
    pub fn get_destination(&self) -> &Point{
        &self.dest
    }

    pub fn set_destination(&mut self, point: &Point){
        self.dest = *point;
    }
}

impl Entity for PacketEntity{
    fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, 1.5, RED);
    }

    fn get_position(&self) -> &Point {
        &self.pos
    }

    fn draw_tooltip(&self, _: &egui::Context) {
        
    }

    fn move_to(&mut self, point: &Point) {
        self.pos = *point;
    }

    fn draw_debug(&self){
        draw_line(self.pos.x, self.pos.y, self.dest.x, self.dest.y, 1.0, BLUE);
        draw_line(self.pos.x, self.pos.y, self.pos.x+(self.velocity.x*10.0), self.pos.y+(self.velocity.y*10.0), 3.0, RED);
        draw_line(self.src.x, self.src.y, self.pos.x, self.pos.y, 1.0, GREEN);
    }
    fn update(&mut self){
        let vec_to_dest_unit = self.pos.get_unit_vector(&self.dest);
        let vec_velocity_unit = self.velocity.get_unit_vector();
        let dist_to_dest = self.pos.distance(&self.dest);
        let angle = vec_to_dest_unit.angle(&vec_velocity_unit);
        
        self.velocity = self.velocity.rotate(angle/dist_to_dest);

        self.pos += &self.velocity;
    }

    fn is_point_inside(&self, point: &Point) -> bool {
        let distance = self.pos.distance(point);
        if distance < 1.5{
            return true;
        }
        return false;
    }
}