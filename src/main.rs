use macroquad::prelude::*;
use ui::UI;
use std::sync::mpsc::channel;
use packet_manager::IPPacketInfo;
use network_manager::NetworkManager;
use std::thread;

mod packet_manager;
mod ui;
mod network_manager;
mod math;

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

    thread::spawn(|| {
        let mut network_manager = NetworkManager::new(tx);
        loop {
            network_manager.listen_packets();
        }
    });
    let mut ui = UI::new(rx);
    loop{
        ui.run().await;
    };
}