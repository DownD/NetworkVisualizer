use macroquad::prelude::*;
use std::time::Instant;
use crate::engine::Engine;


pub struct UISettings {
}

impl UISettings {
    pub fn draw(&mut self, start_timestamp: Instant, engine: &mut Engine) {
        let packet_count = engine.get_visible_packet_count();
        let settings = engine.get_settings();
        //draw_text(&format!("Valid packets: {}", engine.get_visible_packet_count()), 10.0, 10.0, 20.0, WHITE);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
            .default_open(false)
            .show(egui_ctx, |ui| {
                ui.add(egui::Slider::new(&mut settings.max_packets, 0..=100000).logarithmic(true).text("Max Packets"));
                ui.label(format!("Number of packets: {:?}", packet_count));
                ui.checkbox(&mut settings.draw_tooltip, "Draw tooltip");
                ui.checkbox(&mut settings.update, "Start/Stop particle update");
                // Checkbox
                ui.checkbox(&mut settings.draw_debug, "Draw debug");
                ui.add(egui::Slider::new(&mut settings.angle_launch, 0.0..=1.57).text("Angle launch"));
                ui.add(egui::Slider::new(&mut settings.speed_launch, 0.0..=2.0).text("Speed launch"));
            });
            
        });
        egui_macroquad::draw();
        draw_text(&format!("FPS: {:?}", 1.0/start_timestamp.elapsed().as_secs_f64()), screen_width()-130.0, 30.0, 20.0, WHITE);
    }
}