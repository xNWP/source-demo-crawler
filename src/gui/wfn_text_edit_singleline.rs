use eframe::{egui::{ Ui, TextEdit, TextBuffer, Sense }, epaint::Color32};

pub fn wfn_text_edit_singleline<'a>(
ui: &mut Ui,
text: &'a mut dyn TextBuffer,
text_colour: Option<Color32>,
draw_background: bool) {
    ui.scope(|ui| {
        if !draw_background {
            ui.style_mut().visuals.extreme_bg_color = Color32::TRANSPARENT;
        }
        let mut te = TextEdit::singleline(text);
        if let Some(colour) = text_colour {
            te = te.text_color(colour);
        }
        ui.add(te).interact(Sense::click())
        .context_menu(|ui| {
            if ui.button("Copy").clicked() {
                ui.output_mut(|o| o.copied_text = text.take());
                ui.close_menu();
            }
        });
    });
}