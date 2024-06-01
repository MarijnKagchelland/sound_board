pub mod menubar;
pub mod modulation;
pub mod side_panel;

pub trait Page {
    fn show(&mut self, ctx: &egui::Context);
}