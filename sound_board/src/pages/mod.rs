pub mod menubar;
pub mod modulation;
pub mod side_panel;
mod custom_graph;
use crate::audio::*;

pub trait Page {
    fn show(&mut self, ctx: &egui::Context, audio_board: &mut SoundBoard);
}

