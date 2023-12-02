use crate::math::Point;
mod packet;
mod node;

pub use node::Node;
pub use packet::PacketEntity;


pub trait Entity{
    fn draw(&self);
    fn draw_debug(&self);
    fn move_to(&mut self, point: &Point);
    fn draw_tooltip(&self, ctx: &egui::Context);
    fn update(&mut self);
    fn get_position(&self) -> &Point;
    fn is_point_inside(&self, point: &Point) -> bool;
}