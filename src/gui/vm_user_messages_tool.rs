use super::{ Event, ViewModel, vm_protobuf_message_list::ProtobufMessageListViewModel };

use source_demo_tool::demo_file::packet::{
    usermessage::UserMessage,
};

use source_demo_tool::demo_file::ParsedUserMessage;

pub struct UserMessagesToolViewModel {
    pub vm_messages: ProtobufMessageListViewModel<UserMessage>,
}

impl UserMessagesToolViewModel {
    pub fn new(user_messages: Vec<ParsedUserMessage>, tick_interval: f32) -> Self {
        let mut messages = Vec::new();
        let mut frame_indices = Vec::new();
        let mut message_indices = Vec::new();
        let mut ticks = Vec::new();
        for msg in user_messages {
            frame_indices.push(msg.frame_index);
            message_indices.push(msg.message_index);
            messages.push(msg.message_return);
            ticks.push(msg.tick);
        }

        let mut vm_messages
            = ProtobufMessageListViewModel::new("user_messages", messages);
        vm_messages.set_message_header_callback(move |index, ui, events, _| {
            ui.horizontal(|ui| {
                let frame_index = frame_indices[index];
                let msg_index = message_indices[index];
                ui.label(format!("Frame: {}, Message: {}", frame_index + 1, msg_index + 1));
                if ui.button("Goto").clicked() {
                    events.push(Event::SetTool("Frames"));
                    events.push(Event::SelectFrame(frame_index));
                    events.push(Event::SelectMessage("packet_data_messages", msg_index));
                }
            });
        });
        vm_messages.set_tick_column(ticks, tick_interval);
        vm_messages.set_filterable(true);

        Self { vm_messages }
    }
}

impl ViewModel for UserMessagesToolViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<Event>) {
        self.vm_messages.draw(ui, events);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        self.vm_messages.handle_event(event)
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}