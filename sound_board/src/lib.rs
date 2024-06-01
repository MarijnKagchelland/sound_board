use serde::Serialize;
pub use app::AudioMixerApp;
pub use log_system::*;

mod app;
pub mod log_system;
mod pages;
pub mod audio;
pub trait Module {
    fn update(&mut self, ctx: &egui::Context);
}

