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

use math::{Point,Vector};

fn window_conf() -> Conf {
    Conf {
        window_title: "egui with macroquad".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

async fn test_engine(){

    let p1: Point = Point{x:610.0,y:168.0};
    let p2: Point = Point{x:719.0,y:390.0};
    let p3: Point = Point{x:769.0,y:504.0};

    let vec1: Vector = p1.get_unit_vector(&p2);
    let vec2: Vector = p2.get_unit_vector(&p3);

    println!("Vector:{:?} | {:?}",vec1,vec1.tan());
    println!("Vector:{:?} | {:?}",vec2,vec2.tan());
    println!("{:?} ",vec1.dot(&vec2));

    let mut flag: bool = true;

    loop {
        if flag{
            clear_background(BLACK);
            draw_circle(p1.x, p1.y, 10.0, YELLOW);
            draw_circle(p2.x, p2.y, 10.0, YELLOW);
            draw_circle(p3.x, p3.y, 10.0, YELLOW);
            draw_line(p1.x, p1.y, p3.x, p3.y, 1.0, RED);
            draw_line(p1.x, p1.y, p1.x + vec1.x*100.0, p1.y + vec1.y*100.0, 3.0, BLUE);
            flag = false;
        }
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
            .show(egui_ctx, |ui| {
                // Button that sets flag to true
                if ui.button("Next Frame").clicked() {
                    flag = true;
                }
            });
        });

        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //test_engine().await;
    //return;
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