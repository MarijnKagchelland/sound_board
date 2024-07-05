use egui::{Context, menu, TopBottomPanel};
use crate::log_system::*;
use crate::pages::Page;
use crate::audio::SoundBoard;

#[derive(Default)]
pub struct MenuBar {
}

impl Page for MenuBar {
    fn show(&mut self, ctx: &Context, audio_board: &mut SoundBoard) {
        TopBottomPanel::top("Menubar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Update hardware devices list").clicked() {
                        info!("Update device list");
                        let _ = audio_board.update_hardware_devices();
                    }
                    if ui.button("Save").clicked() {
                        info!("Setup saved");
                    }
                    ui.separator();
                    if ui.button("Settings").clicked() {
                        info!("Settings");
                    }
                });
            });
        });
    }
}
