use super::{ Event, ViewModel };
use eframe::egui;
use source_demo_tool::demo_file::FullGameEvent;

pub struct GameEventsToolViewModel {

}

impl GameEventsToolViewModel {
    pub fn new(game_events: Vec<FullGameEvent>) -> Self {


        Self { }
    }
}

impl ViewModel for GameEventsToolViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>) {
        ui.heading("Hello from GameEventsToolViewModel");
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}