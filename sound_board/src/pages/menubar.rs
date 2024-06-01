use egui::{Context, menu, TopBottomPanel};
use crate::*;
use crate::pages::Page;


#[derive(Default)]
pub struct MenuBar {

}

impl Page for MenuBar {
    fn show(&mut self, ctx: &Context) {
        TopBottomPanel::top("Menubar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Menu", |ui| {
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