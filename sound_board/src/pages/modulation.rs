use egui::{CentralPanel, Context, ScrollArea, Ui};

use crate::log::*;
use crate::pages::Page;

#[derive(Default)]
pub struct Modulation {

}

impl Page for Modulation {
    fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {

        });
    }
}