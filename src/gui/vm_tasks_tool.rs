use eframe::{egui::{ self, Layout, TextStyle, RichText}, emath::Align, epaint::Vec2};

use super::{ Event, ViewModel };
use std::sync::mpsc;


pub struct TasksToolViewModel {

}

impl ViewModel for TasksToolViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<Event>) {
        ui.with_layout(
            Layout::top_down(Align::Center),
            |ui| {
                let button_size: Vec2 = [320.0, 48.0].into();

                ui.add_space(15.0);

                ui.label(RichText::new("Tasks").size(48.0).italics());

                ui.add_space(7.5);
                ui.scope(|ui| {
                    ui.set_width(280.0);
                    ui.separator();
                });
                ui.add_space(15.0);

                if ui.add(
                    egui::Button::new("Dump all NetMessage warnings/errors to console")
                    .min_size(button_size)
                ).clicked() {
                    // vm_main handles this
                    events.push(Event::EmitNetMsgWarnErrs);
                }

                if ui.add(egui::Button::new("Dump all UserMessage warnings/errors to console")
                    .min_size(button_size)
                ).clicked() {
                    // vm_main handles this
                    events.push(Event::EmitUserMsgWarnErrs);
                }
            }
        );
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

pub struct TaskRunningViewModel {
    task_name: &'static str,
    rx_sub_msg: mpsc::Receiver<String>,
    last_sub_msg: String,
}

impl TaskRunningViewModel {
    pub fn new(task_name: &'static str, rx_sub_msg: mpsc::Receiver<String>) -> Self {
        Self {
            task_name,
            rx_sub_msg,
            last_sub_msg: "".into(),
        }
    }
}

impl ViewModel for TaskRunningViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, _events: &mut Vec<Event>) {
        // check for update percent message
        match self.rx_sub_msg.try_recv() {
            Ok(v) => self.last_sub_msg = v,
            Err(_) => {}
        }

        // draw ui
        const SPINNER_SIZE: f32 = 48.0;
        let vertical_spacing
            = (ui.available_height()
            - SPINNER_SIZE
            - 2.0 * ui.spacing().item_spacing.y
            - 2.0 * ui.text_style_height(&TextStyle::Body)
            ) / 2.0;

        ui.vertical_centered(|ui| {
            ui.style_mut().wrap = Some(false);
            ui.add_space(vertical_spacing);
            ui.add(egui::Spinner::new().size(SPINNER_SIZE));
            ui.label(self.task_name);
            ui.label(self.last_sub_msg.as_str());
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}