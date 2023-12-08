use crate::data::IPPacketInfo;

mod channel_listener;
mod engine;

pub use engine::Engine;

pub struct EngineSettings{
    pub update: bool,
    pub draw_debug: bool,
    pub max_packets: u32,

    pub angle_launch: f32,
    pub speed_launch: f32,

    pub delete_distance: f32
}

pub trait PhysicsEngine {
    fn get_settings(&mut self) -> &mut EngineSettings;
    fn add_packet(&mut self, packet: &IPPacketInfo);
    fn update(&mut self);
}

pub trait GraphicsEngine {
    fn get_settings(&mut self) -> &mut EngineSettings;
    fn draw(&self);
}
