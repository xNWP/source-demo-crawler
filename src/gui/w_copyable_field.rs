use eframe::egui::{ self, Widget, CursorIcon, Sense };

pub struct CopyableFieldWidget {
    pub label: String,
    pub value: Box<dyn ToString>,
}

impl Widget for CopyableFieldWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label(self.label);
            if ui.code(self.value.to_string())
                .interact(Sense::click())
                .on_hover_cursor(CursorIcon::PointingHand)
                .on_hover_text_at_pointer("Copy")
                .clicked() {
                    ui.output_mut(|o| {
                        o.copied_text = self.value.to_string();
                    });
                }
        }).response
    }
}