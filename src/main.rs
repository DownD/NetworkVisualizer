mod packet_manager;
mod ui;
mod network_manager;
mod math;

use macroquad::prelude::*;
use ui::UI;
use std::sync::mpsc::channel;
use packet_manager::IPPacketInfo;
use network_manager::NetworkManager;
use std::thread;

#[macroquad::main("GUI test")]
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