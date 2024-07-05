use egui::{Button, Context, ScrollArea, Vec2};
use crate::log_system::*;
use crate::pages::Page;
use crate::audio::SoundBoard;

#[derive(Default)]
pub struct SidePanel {
    
}

impl Page for SidePanel {
    fn show(&mut self, ctx: &Context, audio_board: &mut SoundBoard) {
        egui::SidePanel::new(egui::panel::Side::Left, 
            "Output channel list main window")
            .exact_width(180.0)
            .show(ctx, |ui| {

            ui.heading("Audio output channels");
            ScrollArea::vertical()
                .stick_to_right(true)
                .show(ui, |ui| {
                
                let vec = audio_board.get_hardware_output_list();
                for device in vec {
                    if ui.add_sized(Vec2{x: 180., y: 60.}, 
                        Button::new(format!("{}", device))).clicked() {

                    }
                }
            });
        });
    }
}

