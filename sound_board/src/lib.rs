//use serde::Serialize;
pub use app::AudioMixerApp;
pub use log_system::*;

mod app;
mod pages;
pub mod basic_audio_driver_rust;
pub mod log_system;
pub mod audio;

pub trait Module {
    fn update(&mut self, ctx: &egui::Context);
}

