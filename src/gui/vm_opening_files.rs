use super::{ Event, ViewModel };
use eframe::egui::{ self, TextStyle };

pub struct OpeningFileViewModel {
    name: String
}

impl OpeningFileViewModel {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl ViewModel for OpeningFileViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, _events: &mut Vec<Event>) {
        const SPINNER_SIZE: f32 = 30.0;
        let vertical_spacing
            = (ui.available_height()
            - SPINNER_SIZE
            - ui.spacing().item_spacing.y
            - ui.text_style_height(&TextStyle::Body)
            ) / 2.0;

        ui.vertical_centered(|ui| {
            ui.style_mut().wrap = Some(false);
            ui.add_space(vertical_spacing);
            ui.add(egui::Spinner::new().size(SPINNER_SIZE));
            ui.label(format!("Opening: {}", self.name));
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}