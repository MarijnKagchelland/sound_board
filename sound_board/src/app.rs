use std::sync::Arc;
use std::default::Default;

use egui::{Context, Vec2, viewport};
use eframe::{Frame, HardwareAcceleration, NativeOptions, Renderer, Storage, Theme};

use crate::log_system::log::*;
use crate::pages::menubar::MenuBar;
use crate::pages::side_panel::SidePanel;
use crate::pages::modulation::Modulation;
use crate::pages::*;

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

                inner_size: Some(Vec2{x: 1280., y: 720.}), 
                min_inner_size: Some(Vec2{x: 1280., y: 720.}),
                max_inner_size: Some(Vec2{x: 1280., y: 720.}),
                //min_inner_size: None,
                //max_inner_size: None,
                fullscreen: Some(false),
                maximized: Some(false),
                resizable: Some(true),
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
    sound_board:        crate::audio::SoundBoard,
    menubar_page:       MenuBar,
    side_panel_page:    SidePanel,
    modulation_page:    Modulation,
}

impl AudioMixerApp {
    pub fn start() {
        crate::info!("Starting application");
        eframe::run_native("Audio mixer app",
            ApplicationOptions::options().unwrap(),
            Box::new(|cc| Box::new(AudioMixerApp::new(cc)))).unwrap();
        /* Will never get to this point */
    }
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for AudioMixerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.menubar_page.show(ctx, &mut self.sound_board);
        self.side_panel_page.show(ctx, &mut self.sound_board);
        self.modulation_page.show(ctx, &mut self.sound_board);
    }
    fn save(&mut self, _storage: &mut dyn Storage) {
        // todo!()
    }
}
