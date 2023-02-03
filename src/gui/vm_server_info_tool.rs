use super::{ Event, ViewModel, vm_protobuf_message::ProtobufMessageViewModel };
use eframe::egui;
use source_demo_tool::demo_file::packet::netmessage::{ServerInfoData, NetMessage};

pub struct ServerInfoViewModel {
    vm_protobuf_message: Option<ProtobufMessageViewModel>,
}

impl ServerInfoViewModel {
    pub fn new(server_info: Option<ServerInfoData>) -> Self {
        let vm_protobuf_message = match server_info {
            Some(si) => {
                let nmsg = NetMessage::ServerInfo(si);
                Some(ProtobufMessageViewModel::new(Box::new(nmsg)))
            },
            None => None
        };

        Self {
            vm_protobuf_message,
        }
    }
}

impl ViewModel for ServerInfoViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>) {
        match &mut self.vm_protobuf_message {
            Some(vm) => {
                vm.draw(ui, events);
            },
            None => {
                ui.heading("No Server Info was found in the demo file.");
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}