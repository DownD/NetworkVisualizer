use macroquad::prelude::*;
use ui::UI;
use std::time::Instant;
use std::sync::mpsc::channel;
use network_manager::NetworkManager;
use std::thread;
use data::IPPacketInfo;
use engine::{PhysicsEngine, GraphicsEngine};

pub mod data;
pub mod entities;
mod ui;
mod network_manager;
mod math;
pub mod engine;

fn window_conf() -> Conf {
    Conf {
        window_title: "egui with macroquad".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let (tx, rx) = channel::<IPPacketInfo>();
    let mut ui = UI{};
    let mut network_manager = NetworkManager::new(tx);
    ui.draw_network_device_menu(&mut network_manager).await;

    thread::spawn(move || {
        loop {
            network_manager.listen_packets();
        }
    });
    let mut engine = engine::Engine::new();
    loop{
        let timestamp = Instant::now();
        rx.try_iter().for_each(|packet| {
            engine.add_packet(&packet);
        });
        clear_background(BLACK);
        engine.update();
        engine.draw();
        ui.draw_settings(timestamp, &mut engine);
        next_frame().await
    };
}