use eframe::{egui::{RichText, Layout}, emath::Align};

use super::{ Event, ViewModel };

pub struct AboutHelpViewModel {

}

impl ViewModel for AboutHelpViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, _events: &mut Vec<Event>) {
        ui.with_layout(
            Layout::top_down(Align::Center),
            |ui| {
            ui.add_space(10.0);
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing = [0.0, -8.0].into();
                ui.label(
                    RichText::new("Source Demo Crawler")
                    .size(48.0).italics()
                );
                ui.label(
                    RichText::new(
                        format!(
                            "v{}",
                            env!("CARGO_PKG_VERSION")
                        )
                    ).size(20.0).italics().weak()
                );
            });

            ui.add_space(5.0);

            if ui.link("Source Demo Crawler (GitHub)").clicked() {
                ui.output().open_url("https://github.com/xNWP/source-demo-crawler");
            }
            if ui.link("Source Demo Crawler (crates.io)").clicked() {
                ui.output().open_url("https://crates.io/crates/source-demo-tool-crawler");
            }

            ui.add_space(5.0);
            ui.label(
                RichText::new("Created by Brett 'xNWP' Anthony")
                .size(14.0)
            );
            ui.add_space(2.5);

            if ui.link("Twitter/@ThatNWP").clicked() {
                ui.output().open_url("https://twitter.com/ThatNWP");
            }
            if ui.link("GitHub/@xNWP").clicked() {
                ui.output().open_url("https://github.com/xNWP");
            }

            ui.scope(|ui| {
                ui.set_width(260.0);
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
            });
            ui.label("⬅/➡ to switch between tools.");
            ui.label("⬆/⬇ to move between items in lists.");
            ui.label("Ctrl + ⬆/⬇ to move to the beginning/end of a list.");
            ui.label("Shift + ⬆/⬇ to move 10 items at a time through a list.");
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}