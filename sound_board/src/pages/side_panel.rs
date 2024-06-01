use egui::{Area, Context, Frame, pos2, ScrollArea, Style, Button};
use crate::pages::Page;

const SIZE_OF_LABEL_IN_FRAME: f32 = 50.;

#[derive(Default)]
pub struct SidePanel {

}

impl Page for SidePanel {
    fn show(&mut self, ctx: &Context) {
        egui::SidePanel::new(egui::panel::Side::Left, "Output channel list main window")
            .exact_width(180.0)
            .show(ctx, |ui| {

            ui.heading("Audio output channels");
            ScrollArea::vertical()
                .stick_to_right(true)
                .show(ui, |ui| {

                for i in 0..50 {
                    if ui.add(Button::new(make_name_fit(format!("tester{}", i)))).clicked() {

                    }
                }
            });
        });
    }
}

fn make_name_fit(string: String) -> String {
    let str_len = string.len() as f32;
    let spaces_num = (SIZE_OF_LABEL_IN_FRAME - str_len) / 2.;
    let per_spaces = spaces_num as i32;
    let post_spaces = spaces_num.round() as i32;

    let mut return_string = String::from("");
    for _ in 0..per_spaces {
        return_string.push(' ');
    }
    for ch in string.chars() {
        return_string.push(ch);
    }
    for _ in 0..post_spaces {
        return_string.push(' ');
    }
    return_string
}