use crate::math::Point;
mod node_graphics;
mod packet_graphics;
mod node;

pub use node::Node;
pub use packet_graphics::PacketGraphics;


pub trait Entity{
    fn draw(&self);
    fn draw_debug(&self);
    fn update(&mut self);
    fn get_position(&self) -> &Point;
    fn is_point_inside(&self, point: &Point) -> bool;
}