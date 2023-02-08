use std::{fs::File, io::Read};

use eframe::{egui::{self, RichText, Layout}, emath::Align, epaint::Vec2};

use super::{ Event, ViewModel };

const CHANGELOG_WIDTH: f32 = 640.0;
const SEPARATOR_WIDTH: f32 = 260.0;

pub struct AboutHelpViewModel {
    changelog_text: String,
}

impl AboutHelpViewModel {
    pub fn new() -> Self {
        let changelog_file = File::open("CHANGELOG.md");

        let mut changelog_text = "".to_owned();
        match changelog_file {
            Ok(mut file) => match file.read_to_string(&mut changelog_text) {
                Ok(_) => {},
                Err(e) => eprintln!("Error occured reading changelog: {}", e)
            },
            Err(e) => eprintln!("Error occured opening changelog: {}", e)
        }

        Self { changelog_text }
    }
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
                ui.output_mut(|o| o.open_url("https://github.com/xNWP/source-demo-crawler"));
            }
            if ui.link("Source Demo Crawler (crates.io)").clicked() {
                ui.output_mut(|o| o.open_url("https://crates.io/crates/source-demo-tool-crawler"));
            }

            ui.add_space(5.0);
            ui.label(
                RichText::new("Created by Brett 'xNWP' Anthony")
                .size(14.0)
            );
            ui.add_space(2.5);

            if ui.link("Twitter/@ThatNWP").clicked() {
                ui.output_mut(|o| o.open_url("https://twitter.com/ThatNWP"));
            }
            if ui.link("GitHub/@xNWP").clicked() {
                ui.output_mut(|o| o.open_url("https://github.com/xNWP"));
            }

            ui.scope(|ui| {
                ui.set_width(SEPARATOR_WIDTH);
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
            });
            ui.label("⬅/➡ to switch between tools.");
            ui.label("⬆/⬇ to move between items in lists.");
            ui.label("Ctrl + ⬆/⬇ to move to the beginning/end of a list.");
            ui.label("Shift + ⬆/⬇ to move 10 items at a time through a list.");

            // changelog
            ui.scope(|ui| {
                ui.set_width(SEPARATOR_WIDTH);
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
            });
        });

        let avail_space = ui.available_size();
        ui.horizontal(|ui| {
            ui.set_width(avail_space.x);
            ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.0);
            ui.add_space((avail_space.x - CHANGELOG_WIDTH) / 2.0);

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(CHANGELOG_WIDTH);
                    ui.set_height(avail_space.y - 20.0);
                    egui::ScrollArea::vertical()
                    .max_width(CHANGELOG_WIDTH)
                    .show(ui, |ui| {
                        ui.set_width(CHANGELOG_WIDTH);
                        for line_in in self.changelog_text.lines() {
                            let mut line = line_in.trim();
                            if line.starts_with("###") {
                                line = &line[3..].trim_start();
                                ui.label(RichText::new(line).size(24.0));
                            } else if line.starts_with("#") {
                                line = &line[1..].trim_start();
                                ui.label(RichText::new(line).size(48.0));
                            } else {
                                ui.label(line_in);
                            }
                        }
                    });
                });
            });
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}