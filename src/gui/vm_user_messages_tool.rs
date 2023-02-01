use super::{ Event, ViewModel, vm_protobuf_message_list::ProtobufMessageListViewModel };

use source_demo_tool::demo_file::packet::{
    MessageParseReturn,
    usermessage::UserMessage,
};

pub struct UserMessagesToolViewModel {
    pub vm_messages: ProtobufMessageListViewModel<UserMessage>,
}

impl UserMessagesToolViewModel {
    pub fn new(user_messages: Vec<(usize, MessageParseReturn<UserMessage>)>) -> Self {
        let mut messages = Vec::new();
        let mut frame_indices = Vec::new();
        for msg in user_messages {
            frame_indices.push(msg.0);
            messages.push(msg.1);
        }

        let vm_messages
            = ProtobufMessageListViewModel::new("user_messages", messages, Some(frame_indices));

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