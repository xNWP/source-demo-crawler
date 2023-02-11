use super::{
    Event,
    ViewModel,
    w_copyable_field::CopyableFieldWidget, vm_protobuf_message_list::ProtobufMessageListViewModel,
};
use source_demo_tool::{demo_file::packet::{PacketData, netmessage::{NetMessage, GameEventListData}, self, MessageParseReturn, usermessage::UserMessage}, protobuf_message::ProtobufMessageEnumTraits};
use eframe::egui;

pub struct PacketDataViewModel {
    pub header: packet::Header,
    pub vm_message_list: ProtobufMessageListViewModel<NetMessage>,
}

impl PacketDataViewModel {
    pub fn new(
        packet_data: PacketData,
        game_event_ld: Option<GameEventListData>,
    ) -> Self {
        let header = packet_data.header;
        let mut vm_message_list
            = ProtobufMessageListViewModel::new("packet_data_messages", packet_data.network_messages);

        let umsg_id_map = UserMessage::get_id_map();
        vm_message_list.set_message_name_callback(move |nmsg| {
            match nmsg {
                NetMessage::UserMessage(umd) => {
                    if let Some(id) = umd.msg_type {
                        let id = id as usize;
                        if umsg_id_map.contains_key(&id) {
                            return format!("UserMessage({})", umsg_id_map[&id])
                        }
                    }
                },
                NetMessage::GameEvent(ged) => {
                    if let Some(id) = ged.event_id {
                        if let Some(ge_ld) = &game_event_ld {
                            for k in &ge_ld.Descriptors {
                                if let Some(q) = k.event_id {
                                    if q == id {
                                        if let Some(name) = &k.name {
                                            return format!("GameEvent({})", name)
                                        }
                                        break
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            return nmsg.to_str().to_owned()
        });

        Self {
            header,
            vm_message_list,
        }
    }

    pub fn set_message_header_callback(&mut self, callback: impl Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<NetMessage>) + Send + 'static) {
        self.vm_message_list.set_message_header_callback(callback);
    }

    pub fn set_message_footer_callback(&mut self, callback: impl Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<NetMessage>) + Send + 'static) {
        self.vm_message_list.set_message_footer_callback(callback);
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