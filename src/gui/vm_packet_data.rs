use super::{
    Event,
    ViewModel,
    w_copyable_field::CopyableFieldWidget, vm_protobuf_message_list::ProtobufMessageListViewModel,
};
use source_demo_tool::demo_file::packet::{PacketData, netmessage::NetMessage, self, MessageParseReturn};
use eframe::egui;

pub struct PacketDataViewModel {
    pub header: packet::Header,
    pub vm_message_list: ProtobufMessageListViewModel<NetMessage>,
}

impl PacketDataViewModel {
    pub fn new<F>(packet_data: PacketData, header_callback: F) -> Self
    where F: Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<NetMessage>) + 'static {
        let header = packet_data.header;
        let mut vm_message_list
            = ProtobufMessageListViewModel::new("packet_data_messages", packet_data.network_messages);
        vm_message_list.set_message_header_callback(header_callback);

        Self {
            header,
            vm_message_list,
        }
    }
}

impl ViewModel for PacketDataViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>) {
        let header = &self.header;
        // command info
        //   flags
        //   view origin
        //   view angles
        //   local view angles
        //   int view origin
        //   int view angles
        //   int local view angles
        // data len
        // in seq -> out seq

        // comand info
        ui.group(|ui| {
            ui.heading("Command Info");
            ui.add(CopyableFieldWidget {
                label: "Flags".into(),
                value: Box::new(format!("0x{:0>2x}", header.command_info.flags)),
            });
            egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add(CopyableFieldWidget {
                        label: "View Origin".into(),
                        value: Box::new(header.command_info.view_origin.clone()),
                    });
                    ui.add(CopyableFieldWidget {
                        label: "View Angles".into(),
                        value: Box::new(header.command_info.view_angles.clone()),
                    });
                    ui.add(CopyableFieldWidget {
                        label: "Local View Angles".into(),
                        value: Box::new(header.command_info.local_view_angles.clone()),
                    });
                });
                ui.vertical(|ui| {
                    ui.add(CopyableFieldWidget {
                        label: "Interpolated VO".into(),
                        value: Box::new(header.command_info.inter_view_origin.clone()),
                    });
                    ui.add(CopyableFieldWidget {
                        label: "Interpolated VA's".into(),
                        value: Box::new(header.command_info.inter_view_angles.clone()),
                    });
                    ui.add(CopyableFieldWidget {
                        label: "Interpolated LVA's".into(),
                        value: Box::new(header.command_info.inter_local_view_angles.clone()),
                    });
                });
            });
        });

        // rest of header
        // data len
        ui.add(CopyableFieldWidget {
            label: "Data Length".into(),
            value: Box::new(header.data_length),
        });
        // in + out seq
        ui.horizontal(|ui| {
            ui.add(CopyableFieldWidget {
                label: "In Seq".into(),
                value: Box::new(header.in_seq),
            });
            ui.add(CopyableFieldWidget {
                label: "Out Seq".into(),
                value: Box::new(header.out_seq),
            });
        });

        // Net Messages
        ui.separator();
        ui.heading("Net Messages");

        self.vm_message_list.draw(ui, events);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        if self.vm_message_list.handle_event(event) {
            return true
        }
        false
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}