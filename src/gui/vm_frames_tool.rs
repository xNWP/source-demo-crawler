use std::collections::BTreeMap;

use super::{
    Event,
    ViewModel,
    Focusable,
    vm_packet_data::PacketDataViewModel,
    vm_demo_file::tick_to_time_string,
    table_constants,
};
use source_demo_tool::demo_file::{
    frame::{ Command, Frame }, packet::netmessage::NetMessage,
};
use eframe::egui::{ self, CursorIcon, RichText, Sense };
use egui_extras::{ TableBuilder, Column };

const FRAMES_PLAYER_SLOT_WIDTH: f32 = 80.0;
const MAX_FRAMES_LIST_WIDTH: f32 = 500.0;

pub struct FramesToolViewModel {
    pub vm_frames_list: FramesListViewModel,
    pub vm_packet_data: Option<PacketDataViewModel>,
    last_message_index: Option<usize>,
    frame_data: Vec<FrameData>,
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
    pub fn new(demo_frames: Vec<Frame>, tick_interval: f32) -> Self {
        let mut frame_data = Vec::new();
        let mut user_message_it = 0;
        let mut game_event_it = 0;
        for f in &demo_frames {
            let mut um_index = BTreeMap::new();
            let mut ge_index = BTreeMap::new();

            if let Command::Packet(pd) = &f.command {
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
            vm_frames_list: FramesListViewModel::new(demo_frames, tick_interval),
            vm_packet_data: None,
            last_message_index: None,
            frame_data,
        }
    }

    pub fn select_frame(&mut self, index: usize) -> bool {
        let msg_len = self.vm_frames_list.demo_frames.len();
        if index >= msg_len {
            return false
        }

        if let Some(pd_vm) = &self.vm_packet_data {
            let last_msg = *pd_vm.vm_message_list.get_active_message();
            if let Some(last_index) = last_msg {
                self.last_message_index = Some(last_index);
            }
        }

        self.vm_frames_list.set_active_frame(index);
        let active_message = &self.vm_frames_list.demo_frames[index];
        if let Command::Packet(pd) = &active_message.command {
            let frame_data = self.frame_data[index].clone();

            let mut packet_data = PacketDataViewModel::new(
                pd.clone(),
                move |nmsg_index, ui, events, msg| {
                    if let Some(nmsg) = &msg.message {
                        if let NetMessage::UserMessage(_) = nmsg {
                            let user_message_index = frame_data.user_message_index.as_ref().unwrap();
                            let user_message_index = user_message_index[&nmsg_index];

                            ui.horizontal(|ui| {
                                ui.label(format!("User Message Index: {}", user_message_index));
                                if ui.button("Goto").clicked() {
                                    events.append(&mut vec![
                                        Event::SetTool("User Messages"),
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
                                        Event::SelectGameEvent(game_event_index)
                                    ]);
                                }
                            });
                        }
                    }
                }
            );

            if let Some(index) = self.last_message_index {
                if index < packet_data.vm_message_list.messages.len() {
                    packet_data.vm_message_list.set_active_message(index);
                }
            }

            self.vm_packet_data = Some(packet_data);
        } else {
            self.vm_packet_data = None;
        }

        return true
    }

    pub fn next_frame(&mut self) -> bool {
        if let Some(index) = self.vm_frames_list.active_frame {
            self.select_frame(index + 1)
        } else {
            self.select_frame(0)
        }
    }

    pub fn prev_frame(&mut self) -> bool {
        if let Some(index) = self.vm_frames_list.active_frame {
            self.select_frame(index - 1)
        } else {
            self.select_frame(0)
        }
    }

    pub fn first_frame(&mut self) {
        self.select_frame(0);
    }

    pub fn last_frame(&mut self) {
        self.select_frame(self.vm_frames_list.demo_frames.len() - 1);
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
        });
    }

    fn handle_event(&mut self, event: &super::Event) -> bool {
        if let Event::SelectFrame(index) = event {
            return self.select_frame(*index)
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
    active_frame: Option<usize>,
    b_scroll_next: bool,
}

impl FramesListViewModel {
    pub fn new(demo_frames: Vec<Frame>, tick_interval: f32) -> Self {
        Self {
            tick_interval, demo_frames,
            active_frame: None,
            b_scroll_next: true,
        }
    }
}

impl FramesListViewModel {
    fn set_active_frame(&mut self, index: usize) {
        self.active_frame = Some(index);
        self.b_scroll_next = true;
    }
}

impl ViewModel for FramesListViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<super::Event>) {
        // draw ui
        let mut table_builder = TableBuilder::new(ui);

        if self.b_scroll_next {
            table_builder = table_builder.scroll_to_row(self.active_frame.unwrap_or(0), None);
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
                self.demo_frames.len(), |index, mut row| {
                    let frame = &self.demo_frames[index];
                    let mut responses = Vec::new();
                    let is_active_frame = match self.active_frame {
                        Some(active_index) => index == active_index,
                        None => false
                    };

                    responses.push(row.col(|ui| {
                        let frame = format!("{}", index + 1);
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
                        events.push(Event::SelectFrame(index));
                        events.push(Event::SetFocus(Focusable::FramesListViewModel));
                    }
                });
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}