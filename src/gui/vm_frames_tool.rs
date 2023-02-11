use std::collections::BTreeMap;

use super::{
    Event,
    ViewModel,
    Focusable,
    vm_packet_data::PacketDataViewModel,
    vm_demo_file::tick_to_time_string,
    table_constants, Filters, vm_data_tables::DataTablesViewModel,
};
use source_demo_tool::{demo_file::{
    frame::{ Command, Frame }, packet::{netmessage::{NetMessage, GameEventListData}, MessageParseReturn},
}, protobuf_message::ProtobufMessageEnumTraits};
use eframe::{egui::{ self, CursorIcon, RichText, Sense }, epaint::ColorImage};
use egui_extras::{ TableBuilder, Column, RetainedImage };

const FRAMES_PLAYER_SLOT_WIDTH: f32 = 80.0;
const MAX_FRAMES_LIST_WIDTH: f32 = 500.0;

pub struct FramesToolViewModel {
    pub vm_frames_list: FramesListViewModel,
    pub vm_packet_data: Option<PacketDataViewModel>,
    pub vm_data_tables: Option<DataTablesViewModel>,
    last_message_index: Option<usize>,
    frame_data: Vec<FrameData>,
    last_hide_none_values: bool,
    game_event_ld: Option<GameEventListData>,
    name: &'static str,
}

#[derive(Clone)]
struct FrameData {
    user_message_index: Option<BTreeMap<usize, usize>>,
    game_event_index: Option<BTreeMap<usize, usize>>,
}

impl FrameData {
    pub fn none() -> Self {
        Self {
            user_message_index: None,
            game_event_index: None,
        }
    }
}

impl FramesToolViewModel {
    pub fn new(
        name: &'static str,
        demo_frames: Vec<Frame>,
        tick_interval: f32,
        game_event_ld: Option<GameEventListData>
    ) -> Self {
        let mut frame_data = Vec::new();
        let mut user_message_it = 0;
        let mut game_event_it = 0;
        for f in &demo_frames {
            let mut um_index = BTreeMap::new();
            let mut ge_index = BTreeMap::new();

            if let Command::Packet(pd) | Command::SignOn(pd) = &f.command {
                for nmsg_it in 0..pd.network_messages.len() {
                    let nmsg_return = &pd.network_messages[nmsg_it];
                    if let Some(nmsg) = &nmsg_return.message {
                        if let NetMessage::UserMessage(_) = nmsg {
                            um_index.insert(nmsg_it, user_message_it);
                            user_message_it += 1;
                        } else
                        if let NetMessage::GameEvent(_) = nmsg {
                            ge_index.insert(nmsg_it, game_event_it);
                            game_event_it += 1;
                        }
                    }
                }
            }

            let mut fdata = FrameData::none();
            if !um_index.is_empty() {
                fdata.user_message_index = Some(um_index);
            }
            if !ge_index.is_empty() {
                fdata.game_event_index = Some(ge_index);
            }

            frame_data.push(fdata);
        }

        Self {
            name,
            frame_data,
            game_event_ld,
            vm_frames_list: FramesListViewModel::new(demo_frames, tick_interval, name),
            vm_packet_data: None,
            vm_data_tables: None,
            last_message_index: None,
            last_hide_none_values: false,
        }
    }

    pub fn select_frame(&mut self, index: usize) -> bool {
        let msg_len = self.vm_frames_list.demo_frames.len();
        if index >= msg_len {
            return false
        }

        if self.vm_frames_list.display_frames.binary_search(&index).is_err() {
            self.vm_frames_list.clear_filters();
        }

        self.vm_frames_list.set_active_frame(index);
        let active_message = &self.vm_frames_list.demo_frames[index];

        if let Command::Packet(pd) | Command::SignOn(pd) = &active_message.command {
            let frame_data = self.frame_data[index].clone();

            // carry over "hide None values" + last message index
            if let Some(pd_vm) = &self.vm_packet_data {
                if let Some(pbm_vm) = &pd_vm.vm_message_list.vm_protobuf_message {
                    self.last_hide_none_values = pbm_vm.hide_none_values_get();
                }
                let last_msg = *pd_vm.vm_message_list.get_active_message();
                if let Some(last_index) = last_msg {
                    self.last_message_index = Some(last_index);
                }
            }

            let mut packet_data = PacketDataViewModel::new(
                pd.clone(),
                self.game_event_ld.clone()
            );
            packet_data.set_message_header_callback(move |nmsg_index, ui: &mut egui::Ui, events: &mut Vec<Event>, msg: &MessageParseReturn<NetMessage>| {
                if let Some(nmsg) = &msg.message {
                    if let NetMessage::UserMessage(_) = nmsg {
                        let user_message_index = frame_data.user_message_index.as_ref().unwrap();
                        let user_message_index = user_message_index[&nmsg_index];

                        ui.horizontal(|ui| {
                            ui.label(format!("User Message Index: {}", user_message_index));
                            if ui.button("Goto").clicked() {
                                events.append(&mut vec![
                                    Event::SetTool("User Messages"),
                                    Event::ClearFilter(Filters::UserMessages),
                                    Event::SelectMessage("user_messages", user_message_index)
                                ]);
                            }
                        });
                    } else
                    if let NetMessage::GameEvent(_) = nmsg {
                        let game_event_index = frame_data.game_event_index.as_ref().unwrap();
                        let game_event_index = game_event_index[&nmsg_index];

                        ui.horizontal(|ui| {
                            ui.label(format!("Game Event Index: {}", game_event_index));
                            if ui.button("Goto").clicked() {
                                events.append(&mut vec![
                                    Event::SetTool("Game Events"),
                                    Event::ClearFilter(Filters::GameEvents),
                                    Event::SelectGameEvent(game_event_index)
                                ]);
                            }
                        });
                    }
                }
            });
            packet_data.set_message_footer_callback(move |_, ui: &mut egui::Ui, _, msg: &MessageParseReturn<NetMessage>| {
                if let Some(nmsg) = &msg.message {
                    if let NetMessage::AvatarData(ad) = nmsg {
                        let image = ColorImage::from_rgb(
                            [64, 64],
                            ad.rgb_bytes.as_ref().unwrap().as_slice()
                        );
                        let retained_image = RetainedImage::from_color_image("PlayerAvatar", image);
                        retained_image.show(ui);
                    }
                }
            });

            if let Some(index) = self.last_message_index {
                if index < packet_data.vm_message_list.messages.len() {
                    packet_data.vm_message_list.set_active_message(index);
                    let pbm_vm
                        = packet_data.vm_message_list
                        .vm_protobuf_message.as_mut();
                    if let Some(vm) = pbm_vm {
                        vm.hide_none_values_set(self.last_hide_none_values);
                    }
                }
            }

            self.vm_packet_data = Some(packet_data);
        } else {
            self.vm_packet_data = None;
        }

        if let Command::DataTables(dtd) = &active_message.command {
            self.vm_data_tables = Some(DataTablesViewModel::new(dtd.clone()));
        } else {
            self.vm_data_tables = None;
        }

        return true
    }

    pub fn next_frame(&mut self) {
        self.vm_frames_list.next_frame();
        if let Some(i) = self.vm_frames_list.active_frame {
            self.select_frame(i);
        }
    }

    pub fn prev_frame(&mut self) {
        self.vm_frames_list.prev_frame();
        if let Some(i) = self.vm_frames_list.active_frame {
            self.select_frame(i);
        }
    }

    pub fn first_frame(&mut self) {
        self.vm_frames_list.first_frame();
        if let Some(i) = self.vm_frames_list.active_frame {
            self.select_frame(i);
        }
    }

    pub fn last_frame(&mut self) {
        self.vm_frames_list.last_frame();
        if let Some(i) = self.vm_frames_list.active_frame {
            self.select_frame(i);
        }
    }
}

impl ViewModel for FramesToolViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<super::Event>) {
        // draw ui
        let avail_space = ui.available_size();

        let mut total_panels = 1;
        let active_is_packet = self.vm_packet_data.is_some();
        if active_is_packet {
            total_panels += 1;
        }

        let active_is_data_tables = self.vm_data_tables.is_some();
        if active_is_data_tables {
            total_panels += 1;
        }

        let frames_list_width = {
            if total_panels == 1 {
                avail_space.x
            } else {
                let auto_size = avail_space.x / total_panels as f32;
                if auto_size > MAX_FRAMES_LIST_WIDTH {
                    MAX_FRAMES_LIST_WIDTH
                } else {
                    auto_size
                }
            }
        };
        let packet_data_width = avail_space.x - frames_list_width;

        egui::Grid::new(ui.next_auto_id())
        .show(ui, |ui| {
            // frames list
            ui.vertical(|ui| {
                ui.set_width(frames_list_width);
                ui.set_height(avail_space.y - table_constants::BOTTOM_MARGIN);
                self.vm_frames_list.draw(ui, events);
            });

            // packet
            if active_is_packet {
                ui.vertical(|ui| {
                    ui.set_width(packet_data_width);
                    ui.set_height(avail_space.y - table_constants::BOTTOM_MARGIN);
                    self.vm_packet_data.as_mut().unwrap().draw(ui, events);
                });
            }

            // data tables
            if active_is_data_tables {
                ui.vertical(|ui| {
                    ui.set_width(packet_data_width);
                    ui.set_height(avail_space.y - table_constants::BOTTOM_MARGIN);
                    self.vm_data_tables.as_mut().unwrap().draw(ui, events);
                });
            }
        });
    }

    fn handle_event(&mut self, event: &super::Event) -> bool {
        if let Event::SelectFrame(tool_name, index) = event {
            if self.name == *tool_name {
                return self.select_frame(*index)
            }
        }

        if self.vm_frames_list.handle_event(event) {
            return true
        }
        if let Some(pd_vm) = self.vm_packet_data.as_mut() {
            if pd_vm.handle_event(event) {
                return true
            }
        }
        false
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

pub struct FramesListViewModel {
    tick_interval: f32,
    demo_frames: Vec<Frame>,
    display_frames: Vec<usize>,
    active_frame: Option<usize>,
    b_scroll_next: bool,
    frame_tool_name: &'static str,
    filterable_commands: BTreeMap<u8, (String, bool, usize)>,
    filterable_net_messages: BTreeMap<u64, (String, bool, usize)>,
}

impl FramesListViewModel {
    pub fn new(
        demo_frames: Vec<Frame>,
        tick_interval: f32,
        frame_tool_name: &'static str
    ) -> Self {
        let mut display_frames = Vec::new();
        display_frames.resize(demo_frames.len(), 0);
        let mut filterable_commands = BTreeMap::new();
        let mut filterable_net_messages = BTreeMap::new();

        for i in 0..demo_frames.len() {
            display_frames[i] = i;

            let command = &demo_frames[i].command;
            filterable_commands.entry(command.as_u8())
            .and_modify(|(_, _, count)| {
                *count += 1;
            })
            .or_insert((
                command.get_command_str().to_owned(),
                true,
                1 as usize
            ));

            let command = &demo_frames[i].command;

            if let Command::Packet(pd) | Command::SignOn(pd)
                = command {
                    for nmsg_ret in &pd.network_messages {
                        if let Some(nmsg) = &nmsg_ret.message {
                            filterable_net_messages.entry(nmsg.as_u64())
                            .and_modify(|(_, _, count)| {
                                *count += 1;
                            })
                            .or_insert((
                                nmsg.to_str().to_owned(),
                                true,
                                1 as usize
                            ));
                        }
                    }
            }
        }

        Self {
            tick_interval,
            demo_frames,
            frame_tool_name,
            display_frames,
            filterable_commands,
            filterable_net_messages,
            active_frame: None,
            b_scroll_next: true,
        }
    }

    fn set_active_frame(&mut self, index: usize) {
        self.active_frame = Some(index);
        self.b_scroll_next = true;
    }

    fn next_frame(&mut self) {
        match self.active_frame {
            Some(active_index) => {
                if let Ok(i) = self.display_frames.binary_search(&active_index) {
                    let index = i + 1;
                    if index < self.display_frames.len() {
                        self.set_active_frame(self.display_frames[index]);
                    }
                }
            },
            None => self.first_frame()
        }
    }

    fn prev_frame(&mut self) {
        match self.active_frame {
            Some(active_index) => {
                if let Ok(i) = self.display_frames.binary_search(&active_index) {
                    if i > 0 {
                        self.set_active_frame(self.display_frames[i - 1]);
                    }
                }
            },
            None => self.first_frame()
        }
    }

    fn first_frame(&mut self) {
        if let Some(i) = self.display_frames.first() {
            self.set_active_frame(*i);
        }
    }

    fn last_frame(&mut self) {
        if let Some(i) = self.display_frames.last() {
            self.set_active_frame(*i);
        }
    }

    fn update_display_frames(&self) -> Vec<usize> {
        let mut display_frames = Vec::new();
        for i in 0..self.demo_frames.len() {
            let frame = &self.demo_frames[i];
            let command_id = frame.command.as_u8();

            let mut b_display_command = true;
            for (id, (_, display, _)) in &self.filterable_commands {
                if command_id == *id {
                    b_display_command = *display;
                    break;
                }
            }

            let mut b_display_message = false;
            if let Command::Packet(pd) | Command::SignOn(pd) = &frame.command {
                for nmsg_ret in &pd.network_messages {
                    if let Some(nmsg) = &nmsg_ret.message {
                        let nmsg_id = nmsg.as_u64();

                        for (id, (_, display, _)) in &self.filterable_net_messages {
                            if *id == nmsg_id {
                                b_display_message |= *display
                            }
                        }
                    }
                }
            } else {
                b_display_message = true;
            }

            if b_display_command && b_display_message {
                display_frames.push(i);
            }
        }
        display_frames
    }

    fn clear_filters(&mut self) {
        for (_, (_, checked, _)) in &mut self.filterable_commands {
            *checked = true;
        }
        for (_, (_, checked, _)) in &mut self.filterable_net_messages {
            *checked = true;
        }
        self.display_frames = self.update_display_frames();
    }
}

impl ViewModel for FramesListViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<super::Event>) {
        // draw ui
        ui.horizontal(|ui| {
            let mut b_update_display_frames = false;
            ui.label("Filters");
            ui.menu_button("Commands", |ui| {
                for (_, (name, checked, count)) in &mut self.filterable_commands {
                    let mut ck = *checked;
                    if ui.checkbox(&mut ck, format!("{} ({})", name, count)).changed() {
                        b_update_display_frames = true;
                        *checked = ck;
                    }
                }
            });
            ui.menu_button("Net Messages", |ui| {
                for (_, (name, checked, count)) in &mut self.filterable_net_messages {
                    let mut ck = *checked;
                    if ui.checkbox(&mut ck, format!("{} ({})", name, count)).changed() {
                        b_update_display_frames = true;
                        *checked = ck;
                    }
                }
            });

            if b_update_display_frames {
                self.display_frames = self.update_display_frames();
            }
        });

        let mut table_builder = TableBuilder::new(ui);

        if self.b_scroll_next {
            let active_index = self.display_frames.binary_search(
                &self.active_frame
                .unwrap_or(0)
            );
            let active_index = active_index.unwrap_or(0);

            table_builder = table_builder.scroll_to_row(active_index, None);
            self.b_scroll_next = false;
        }

        table_builder
        .striped(true)
        .column(Column::exact(table_constants::COL_INDEX_WIDTH))
        .column(Column::exact(table_constants::COL_TICK_WIDTH))
        .column(Column::exact(table_constants::COL_TIME_WIDTH))
        .column(Column::exact(FRAMES_PLAYER_SLOT_WIDTH))
        .column(Column::remainder())
        .header(table_constants::HEADER_HEIGHT, |mut row| {
            row.col(|ui| {
                ui.label("Frame");
            });
            row.col(|ui| {
                ui.label("Tick");
            });
            row.col(|ui| {
                ui.label("Time");
            });
            row.col(|ui| {
                ui.label("Player Slot");
            });
            row.col(|ui| {
                ui.label("Command");
            });
            })
        .body(|body| {
            body.rows(table_constants::ROW_HEIGHT,
                self.display_frames.len(), |index, mut row| {
                    let frame_index = self.display_frames[index];
                    let frame = &self.demo_frames[frame_index];
                    let mut responses = Vec::new();
                    let is_active_frame = match self.active_frame {
                        Some(active_index) => frame_index == active_index,
                        None => false
                    };

                    responses.push(row.col(|ui| {
                        let frame = format!("{}", frame_index + 1);
                        if is_active_frame {
                            ui.label(RichText::new(frame).color(table_constants::SELECTED_ITEM_COLOUR));
                        } else {
                            ui.label(frame);
                        }
                    }).1);
                    responses.push(row.col(|ui| {
                        let tick = format!("{}", frame.tick);
                        if is_active_frame {
                            ui.label(RichText::new(tick).color(table_constants::SELECTED_ITEM_COLOUR));
                        } else {
                            ui.label(tick);
                        }
                    }).1);
                    responses.push(row.col(|ui| {
                        let time = tick_to_time_string(self.tick_interval, frame.tick);
                        if is_active_frame {
                            ui.label(RichText::new(time).color(table_constants::SELECTED_ITEM_COLOUR));
                        } else {
                            ui.label(time);
                        }
                    }).1);
                    responses.push(row.col(|ui| {
                        let player_slot = format!("{}", frame.player_slot);
                        if is_active_frame {
                            ui.label(RichText::new(player_slot).color(table_constants::SELECTED_ITEM_COLOUR));
                        } else {
                            ui.label(player_slot);
                        }
                    }).1);
                    responses.push(row.col(|ui| {
                        let command = frame.command.get_command_str();
                        if is_active_frame {
                            ui.label(RichText::new(command).color(table_constants::SELECTED_ITEM_COLOUR));
                        } else {
                            ui.label(command);
                        }
                    }).1);

                    let mut is_any_clicked = false;
                    for res in responses {
                        let res = res.interact(Sense::click());

                        is_any_clicked |= res.clicked();

                        res.on_hover_cursor(CursorIcon::PointingHand);
                    }

                    if is_any_clicked {
                        events.append(&mut vec![
                            Event::SelectFrame(self.frame_tool_name, frame_index),
                            Event::SetFocus(Focusable::FramesListViewModel)
                        ]);
                    }
                });
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}