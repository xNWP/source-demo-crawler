use eframe::egui::{ self, Widget };

pub struct CopyableFieldWidget {
    pub label: String,
    pub value: Box<dyn ToString>,
}

impl Widget for CopyableFieldWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let res = ui.horizontal(|ui| {
            ui.label(self.label);
            ui.code(self.value.to_string());
        }).response;

        if res.clicked() {
            // TODO: FIXME: doesn't work :')
            //ui.output().copied_text = "capitalism, more like help".into();
        }
        
        let res2 = res.clone();
        //res.on_hover_cursor(CursorIcon::PointingHand).on_hover_text_at_pointer("Copy");

        res2
    }
}