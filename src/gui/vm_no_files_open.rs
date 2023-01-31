use super::{ Event, ViewModel };
use eframe::egui::{ self, Sense, TextStyle };

pub struct NoFilesOpenViewModel {}

impl ViewModel for NoFilesOpenViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>) {
        let res = ui.vertical_centered(|ui| {
            let vertical_spacing
                = (ui.available_height()
                - ui.text_style_height(&TextStyle::Heading)
                - ui.spacing().item_spacing.y
                - ui.text_style_height(&TextStyle::Body)
                ) / 2.0;

            ui.style_mut().wrap = Some(false);
            ui.add_space(vertical_spacing);
            ui.heading("No files opened");
            ui.label("Double click or press Ctrl+O to open a new file.");
            ui.add_space(vertical_spacing);
        }).response.interact(Sense::click());
        
        if res.double_clicked() {
            events.push(Event::BeginOpenFile);
        }        
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}
