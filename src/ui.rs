use crate::engine::Engine;
use crate::network_manager::NetworkManager;
use egui_extras::{Column, TableBuilder};
use macroquad::prelude::*;
use pcap::Device;
use std::net::IpAddr;
use std::time::Instant;

pub struct UI {}

impl UI {
    pub async fn draw_network_device_menu(&mut self, network_manager: &mut NetworkManager) {
        //transform devices into a hashmap with ids
        let devices : Vec<Device> = NetworkManager::get_network_devices();
        let mut selected_device_idx = 0;
        let mut continue_button = false;
        loop {
            clear_background(BLACK);
            egui_macroquad::ui(|egui_ctx| {
                egui::Window::new("Network Device Selection")
                .title_bar(false)
                .show(egui_ctx, |ui| {
                    ui.vertical_centered(|ui|{
                    ui.heading("Select a network device");
                    ui.separator();
                    TableBuilder::new(ui)
                        .striped(true)
                        .column(Column::auto())
                        .column(Column::initial(300.0).clip(true))
                        .column(Column::initial(300.0))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("");
                            });
                            header.col(|ui| {
                                ui.strong("Description");
                            });
                            header.col(|ui| {
                                ui.strong("IP");
                            });
                        })
                        .body(|mut body| {
                            for (i,device) in devices.iter().enumerate() {
                                let vec: Vec<IpAddr> =
                                    device.addresses.iter().map(|a| a.addr).collect();
                                let mut ip = String::new();
                                for addr in vec {
                                    ip.push_str(&format!("{:?} | ", addr));
                                }
                                body.row(25.0, |mut row| {
                                    row.col(|ui| {
                                        ui.radio_value(&mut selected_device_idx, i, "");
                                    });
                                    row.col(|ui| {
                                        ui.label(device.desc.clone().unwrap_or("".to_string()));
                                    });
                                    row.col(|ui| {
                                        ui.label(ip);
                                    });
                                });
                            }
                        });
                        continue_button = ui.button("Continue").clicked();
                    });
                });
            });
            egui_macroquad::draw();
            next_frame().await;
            if continue_button {
                network_manager.set_device(devices[selected_device_idx].clone());
                break;
            }
        }
    }

    pub fn draw_settings(&mut self, start_timestamp: Instant, engine: &mut Engine) {
        let packet_count = engine.get_visible_packet_count();
        let settings = engine.get_settings();
        //draw_text(&format!("Valid packets: {}", engine.get_visible_packet_count()), 10.0, 10.0, 20.0, WHITE);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings").show(egui_ctx, |ui| {
                ui.add(
                    egui::Slider::new(&mut settings.max_packets, 0..=100000)
                        .logarithmic(true)
                        .text("Max Packets"),
                );
                ui.label(format!("Number of packets: {:?}", packet_count));
                ui.checkbox(&mut settings.draw_tooltip, "Draw tooltip");
                ui.checkbox(&mut settings.update, "Start/Stop particle update");
                // Checkbox
                ui.checkbox(&mut settings.draw_debug, "Draw debug");
                ui.add(
                    egui::Slider::new(&mut settings.angle_launch, 0.0..=1.57).text("Angle launch"),
                );
                ui.add(
                    egui::Slider::new(&mut settings.speed_launch, 0.0..=2.0).text("Speed launch"),
                );
            });
        });
        egui_macroquad::draw();
        draw_text(
            &format!("FPS: {:?}", 1.0 / start_timestamp.elapsed().as_secs_f64()),
            screen_width() - 130.0,
            30.0,
            20.0,
            WHITE,
        );
    }
}
