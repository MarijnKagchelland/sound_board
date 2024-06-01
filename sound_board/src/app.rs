use std::sync::Arc;
use std::fmt::format;
use std::default::Default;

use egui::panel::Side;
use egui::Direction::TopDown;
use egui::Align::{Center, Min};
use egui::{CentralPanel, Context, ScrollArea, SidePanel, Vec2, viewport, Window};
use eframe::{Frame, HardwareAcceleration, NativeOptions, Renderer, Storage, Theme};

use crate::log_system::log::*;
use crate::pages::side_panel::*;
use crate::pages::menubar::MenuBar;
use crate::pages::{Page, side_panel};
use crate::pages::modulation::Modulation;

struct ApplicationOptions;
impl ApplicationOptions {
    fn options() -> Result<NativeOptions, String> {
        let image = include_bytes!("../assets/audioMixer.png");
        let icon = eframe::icon_data::from_png_bytes(image).unwrap();
        Ok(NativeOptions {
            viewport: egui::ViewportBuilder {
                title: Some(String::from("Audio mixer app")),
                app_id: None,
                position: None,

                inner_size: Some(Vec2{x: 1000., y: 800.}),
                min_inner_size: None,
                max_inner_size: None,
                fullscreen: Some(false),
                maximized: Some(false),
                resizable: Some(false),
                transparent: None,
                decorations: None,
                icon: Some(Arc::new(icon)),
                active: None,
                visible: None,
                fullsize_content_view: None,
                title_shown: Some(true),
                titlebar_buttons_shown: None,
                titlebar_shown: Some(true),
                drag_and_drop: None,
                taskbar: Some(true),
                close_button: Some(true),
                minimize_button: Some(true),
                maximize_button: Some(false),
                window_level: Some(viewport::WindowLevel::Normal),
                mouse_passthrough: Some(false),
                window_type: None,
            },

            vsync: true,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: HardwareAcceleration::Preferred,

            renderer: Renderer::default (),
            follow_system_theme: false,
            default_theme: Theme::Dark,
            run_and_return: false, // todo: chance to true when the application is done. This will ensure the app run in the background.

            event_loop_builder: None,
            window_builder: None,
            shader_version: None,
            centered: true,
            persist_window: true,
        })
    }
}

// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)]
#[derive(Default)]
pub struct AudioMixerApp {
    menubar_page: MenuBar,
    side_panel_page: side_panel::SidePanel,
    modulation_page: Modulation,
}

impl AudioMixerApp {
    pub fn start() {
        crate::info!("Starting application");
        eframe::run_native("Audio mixer app",
                           ApplicationOptions::options().unwrap(),
                           Box::new(|cc| Box::new(AudioMixerApp::new(cc)))).unwrap();
        /* will never get to this point */
    }
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
    fn name(&self) -> &str {
        "Audio mixer app"
    }
}

impl eframe::App for AudioMixerApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.menubar_page.show(ctx);
        self.side_panel_page.show(ctx);
        self.modulation_page.show(ctx);
    }
    fn save(&mut self, _storage: &mut dyn Storage) {
        // todo!()
    }
}