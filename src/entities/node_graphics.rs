use crate::math::Point;
use crate::entities::Entity;
use macroquad::prelude::*;

pub struct NodeGraphics{
    pos: Point,
    radius: f32,
}

impl NodeGraphics{
    pub fn new(pos: Point, radius: f32) -> NodeGraphics{
        NodeGraphics{
            pos: pos,
            radius: radius,
        }
    }
}

impl Entity for NodeGraphics{
    fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    fn draw_debug(&self){
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }

    fn get_position(&self) -> &Point {
        &self.pos
    }

    fn update(&mut self) {
        
    }

    fn is_point_inside(&self, point: &Point) -> bool{
        let distance = self.pos.distance(point);
        if distance < self.radius{
            return true;
        }
        return false;
    }
}
