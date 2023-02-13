use std::collections::BTreeMap;

use super::vm_demo_file::tick_to_time_string;
use super::{ Event, ViewModel, Focusable, table_constants };
use eframe::egui::{ self, RichText, Sense, CursorIcon, Layout };
use eframe::emath::Align;
use egui_extras::{ Column, TableBuilder };
use source_demo_tool::protobuf_message::ProtobufMessageEnumTraits;
use super::vm_protobuf_message::ProtobufMessageViewModel;
use source_demo_tool::demo_file::packet::MessageParseReturn;

use super::vm_main::{ print_proto_warns, print_proto_err };

const MESSAGE_LIST_FULL_MAX_WIDTH: f32 = 600.0;
const MESSAGE_LIST_FULL_MIN_WIDTH: f32 = 420.0;
const MESSAGE_LIST_PARTIAL_MAX_WIDTH: f32
    = MESSAGE_LIST_FULL_MAX_WIDTH
    - table_constants::COL_TICK_WIDTH
    - table_constants::COL_TIME_WIDTH;
const MESSAGE_LIST_PARTIAL_MIN_WIDTH: f32
    = MESSAGE_LIST_FULL_MIN_WIDTH
    - table_constants::COL_TICK_WIDTH
    - table_constants::COL_TIME_WIDTH;
const MESSAGE_DETAIL_MIN_WIDTH: f32 = 420.0;

pub struct ProtobufMessageListViewModel<MessageType: ProtobufMessageEnumTraits> {
    pub vm_protobuf_message: Option<ProtobufMessageViewModel>,
    pub messages: BTreeMap<
        &'static str,
        BTreeMap<usize, MessageParseReturn<MessageType>>
    >,
    display_messages: Vec<(usize, MessageParseReturn<MessageType>)>,
    active_message: Option<usize>,
    name: &'static str,
    b_scroll_next: bool,
    message_header_callback: Option<Box<
        dyn Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<MessageType>)
        + Send
    >>,
    message_footer_callback: Option<Box<
        dyn Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<MessageType>)
        + Send
    >>,
    message_ticks: Option<Vec<i32>>,
    tick_interval: Option<f32>,
    filterable: bool,
    active_filter_index: usize,
    filterable_data: Vec<(&'static str, usize)>,
    filterable_list: Vec<String>,
    message_name_callback: Option<Box<
        dyn Fn(&MessageType) -> String
        + Send
    >>,
}

impl<MessageType: ProtobufMessageEnumTraits + Clone + 'static> ProtobufMessageListViewModel<MessageType> {
    pub fn new(name: &'static str, messages_vec: Vec<MessageParseReturn<MessageType>>) -> Self {
        let mut filterable_data = BTreeMap::new();
        let mut messages:
            BTreeMap<
                &'static str,
                BTreeMap<usize, MessageParseReturn<MessageType>>
            > = BTreeMap::new();

        messages.insert("None", BTreeMap::new());

        for i in 0..messages_vec.len() {
            let msg_return = &messages_vec[i];
            if let Some(msg) = &msg_return.message {
                let msg_str = msg.to_str();
                filterable_data
                    .entry(msg_str)
                    .and_modify(|v| {*v += 1})
                    .or_insert(1 as usize);

                messages
                    .entry("None")
                    .and_modify(|m| {
                        m.insert(i, msg_return.clone());
                    });

                messages
                    .entry(msg_str)
                    .and_modify(|m| {
                        m.insert(i, msg_return.clone());
                    })
                    .or_insert({
                        let mut m = BTreeMap::new();
                        m.insert(i, msg_return.clone());
                        m
                    });
            }
        }
        let filterable_data: Vec<(&'static str, usize)> = filterable_data.into_iter().collect();

        let mut filterable_list = Vec::new();
        filterable_list.push("None".to_owned());
        for fd in &filterable_data {
            filterable_list.push(
                format!(
                    "{} ({})",
                    fd.0, fd.1
                )
            );
        }

        let display_messages
            = messages["None"].clone().into_iter().collect();

        Self {
            name,
            messages,
            display_messages,
            filterable_data,
            filterable_list,
            vm_protobuf_message: None,
            active_message: None,
            b_scroll_next: true,
            message_header_callback: None,
            message_footer_callback: None,
            message_ticks: None,
            tick_interval: None,
            filterable: false,
            active_filter_index: 0,
            message_name_callback: None,
        }
    }

    pub fn set_filterable(&mut self, filterable: bool) {
        self.filterable = filterable;
    }

    pub fn set_tick_column(&mut self, ticks: Vec<i32>, tick_interval: f32) {
        self.message_ticks = Some(ticks);
        self.tick_interval = Some(tick_interval);
    }

    pub fn set_message_header_callback<F>(&mut self, callback: F)
    where F: Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<MessageType>)
    + Send + 'static {
        self.message_header_callback = Some(Box::new(callback));
    }

    pub fn set_message_footer_callback<F>(&mut self, callback: F)
    where F: Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<MessageType>)
    + Send + 'static {
        self.message_footer_callback = Some(Box::new(callback));
    }

    pub fn set_message_name_callback<F>(&mut self, callback: F)
    where F: Fn(&MessageType) -> String
    + Send + 'static {
        self.message_name_callback = Some(Box::new(callback));
    }

    pub fn get_active_message(&self) -> &Option<usize> {
        &self.active_message
    }

    pub fn set_active_message(&mut self, index: usize) -> bool {
        for msg in &self.display_messages {
            if msg.0 == index {
                self.b_scroll_next = true;
                self.active_message = Some(index);
                let active_message = &msg.1.message;

                if let Some(warns) = &msg.1.warnings {
                    print_proto_warns(self.name, warns);
                } else if let Some(err) = &msg.1.err {
                    print_proto_err(self.name, err);
                }

                if let Some(msg) = active_message {
                    // carry over prior "hide None values"
                    let hide_none_values = {
                        if let Some(pbm_vm) = &self.vm_protobuf_message {
                            pbm_vm.hide_none_values_get()
                        } else {
                            false
                        }
                    };

                    let mut pbm_vm = ProtobufMessageViewModel::new(Box::new(msg.clone()));
                    pbm_vm.hide_none_values_set(hide_none_values);

                    self.vm_protobuf_message = Some(
                        pbm_vm
                    );
                }
                return true
            }
        }
        false
    }

    pub fn next_message(&mut self) -> bool {
        match &self.active_message {
            Some(_) => {
                let mut b_capture_next = false;
                let mut next_index = None;
                for msg in &self.display_messages {
                    if b_capture_next {
                        next_index = Some(msg.0);
                        break
                    }
                    if msg.0 == self.active_message.unwrap() {
                        b_capture_next = true
                    }
                }
                if let Some(i) = next_index {
                    self.set_active_message(i)
                } else {
                    false
                }
            },
            None => self.first_message()
        }
    }

    pub fn prev_message(&mut self) -> bool {
        match &self.active_message {
            Some(_) => {
                let mut prev_index = None;
                for msg in &self.display_messages {
                    if msg.0 == self.active_message.unwrap() {
                        break
                    }
                    prev_index = Some(msg.0);
                }
                if let Some(i) = prev_index {
                    self.set_active_message(i)
                } else {
                    false
                }
            },
            None => self.first_message()
        }
    }

    pub fn first_message(&mut self) -> bool {
        self.set_active_message(self.display_messages.first().unwrap().0)
    }

    pub fn last_message(&mut self) -> bool {
        self.set_active_message(self.display_messages.last().unwrap().0)
    }

    pub fn clear_filter(&mut self) {
        self.active_filter_index = 0;
        self.display_messages = self.messages["None"].clone().into_iter().collect();
    }
}

impl<MessageType: ProtobufMessageEnumTraits + Clone + 'static> ViewModel for ProtobufMessageListViewModel<MessageType> {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<Event>) {

        let avail_space = ui.available_size();

        egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
            ui.push_id(1, |ui| {
                let message_list_width = {
                    if self.vm_protobuf_message.is_none() {
                        avail_space.x
                    } else {
                        let avail_width = avail_space.x - MESSAGE_DETAIL_MIN_WIDTH;
                        if self.message_ticks.is_some() {
                            if avail_width > MESSAGE_LIST_FULL_MAX_WIDTH {
                                MESSAGE_LIST_FULL_MAX_WIDTH
                            } else if avail_width < MESSAGE_LIST_FULL_MIN_WIDTH {
                                MESSAGE_LIST_FULL_MIN_WIDTH
                            } else {
                                avail_width
                            }
                        } else {
                            if avail_width > MESSAGE_LIST_PARTIAL_MAX_WIDTH {
                                MESSAGE_LIST_PARTIAL_MAX_WIDTH
                            } else if avail_width < MESSAGE_LIST_PARTIAL_MIN_WIDTH {
                                MESSAGE_LIST_PARTIAL_MIN_WIDTH
                            } else {
                                avail_width
                            }
                        }
                    }
                };

                // message list
                ui.vertical(|ui| {
                    ui.set_width(message_list_width);
                    ui.set_height(avail_space.y - table_constants::BOTTOM_MARGIN);

                    if self.filterable {
                        ui.with_layout(
                            Layout::right_to_left(Align::TOP),
                            |ui| {
                                ui.set_width(message_list_width);
                                ui.add_space(20.0);

                                if egui::ComboBox::new(
                                    ui.next_auto_id(),
                                    "Filter"
                                ).width(200.0)
                                .show_index(
                                    ui,
                                    &mut self.active_filter_index,
                                    self.filterable_list.len(),
                                    |i| self.filterable_list[i].clone()
                                )
                                .changed() {
                                    self.display_messages = {
                                        if self.active_filter_index != 0 {
                                            let msg_str = self.filterable_data[self.active_filter_index - 1].0;
                                            self.messages[msg_str].clone().into_iter().collect()
                                        } else {
                                            self.messages["None"].clone().into_iter().collect()
                                        }
                                    };
                                    self.first_message();
                                }
                            }
                        );
                        ui.end_row();
                    }

                    let mut table_builder = TableBuilder::new(ui);

                    if self.b_scroll_next {
                        // get real index
                        let a_index = self.active_message.unwrap_or(0);
                        let mut real_index = 0;
                        for i in 0..self.display_messages.len() {
                            let msg = &self.display_messages[i];
                            if a_index == msg.0 {
                                real_index = i;
                                break
                            }
                        }
                        table_builder = table_builder.scroll_to_row(
                            real_index,
                            None
                        );
                        self.b_scroll_next = false;
                    }

                    table_builder = table_builder.striped(true)
                    .column(Column::exact(table_constants::COL_INDEX_WIDTH));

                    if self.message_ticks.is_some() {
                        table_builder = table_builder
                        .column(Column::exact(table_constants::COL_TICK_WIDTH))
                        .column(Column::exact(table_constants::COL_TIME_WIDTH));
                    }

                    table_builder.column(Column::remainder())
                    .header(table_constants::HEADER_HEIGHT, |mut row| {
                        row.col(|ui| {
                            ui.label("Index");
                        });
                        if self.message_ticks.is_some() {
                            row.col(|ui| {
                                ui.label("Tick");
                            });
                            row.col(|ui| {
                                ui.label("Time");
                            });
                        }
                        row.col(|ui| {
                            ui.label("Name");
                        });
                    })
                    .body(|body| {
                        body.rows(
                            table_constants::ROW_HEIGHT,
                            self.display_messages.len(),
                            |index, mut row| {

                            let message_return_pair = &self.display_messages[index];
                            let name = {
                                if let Some(msg) = &message_return_pair.1.message {
                                    if let Some(f) = &self.message_name_callback {
                                        f(&msg)
                                    } else {
                                        msg.to_str().to_owned()
                                    }
                                } else {
                                    "????".to_owned()
                                }
                            };

                            let real_index = self.display_messages[index].0;
                            let is_active = {
                                if let Some(active_index) = self.active_message {
                                    if real_index == active_index {
                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            };

                            let mut responses = Vec::new();
                            responses.push(row.col(|ui| {
                                let msg = format!("{}", real_index + 1);
                                if is_active {
                                    ui.label(RichText::new(msg).color(table_constants::SELECTED_ITEM_COLOUR));
                                } else {
                                    ui.label(msg);
                                }
                            }).1);

                            if let Some(ticks) = &self.message_ticks {
                                let tick = ticks[real_index];

                                responses.push(row.col(|ui| {
                                    let tick = tick.to_string();
                                    if is_active {
                                        ui.label(RichText::new(tick).color(table_constants::SELECTED_ITEM_COLOUR));
                                    } else {
                                        ui.label(tick);
                                    }
                                }).1);

                                responses.push(row.col(|ui| {
                                    let time = tick_to_time_string(self.tick_interval.unwrap(), tick);
                                    if is_active {
                                        ui.label(RichText::new(time).color(table_constants::SELECTED_ITEM_COLOUR));
                                    } else {
                                        ui.label(time);
                                    }
                                }).1);
                            };

                            responses.push(row.col(|ui| {
                                if is_active {
                                    ui.label(RichText::new(name).color(table_constants::SELECTED_ITEM_COLOUR));
                                } else {
                                    ui.label(name);
                                }
                            }).1);

                            for res in responses {
                                if res
                                .interact(Sense::click())
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked() {
                                    self.set_active_message(real_index);
                                    events.push(Event::SetFocus(Focusable::ProtobufMessageListViewModel(self.name)));
                                }
                            }
                        });
                    });
                });

                // message detail
                ui.vertical(|ui| {
                    ui.set_width(avail_space.x - message_list_width);
                    ui.set_height(avail_space.y - table_constants::BOTTOM_MARGIN);

                    if let Some(pm_vm) = self.vm_protobuf_message.as_mut() {
                        if let Some(msg_header_cb) = &mut self.message_header_callback {
                            let active_index = self.active_message.unwrap();
                            let msg_ref = &self.messages["None"][&active_index];
                            msg_header_cb(active_index, ui, events, msg_ref);
                        }

                        const FOOTER_HEIGHT: f32 = 200.0;
                        let avail_height = ui.available_height();

                        ui.vertical(|ui| {
                            if self.message_footer_callback.is_some() {
                                ui.set_height(avail_height - FOOTER_HEIGHT);
                            }
                            pm_vm.draw(ui, events);
                        });

                        ui.vertical(|ui| {
                            ui.set_height(FOOTER_HEIGHT);
                            if let Some(msg_header_cb) = &mut self.message_footer_callback {
                                let active_index = self.active_message.unwrap();
                                let msg_ref = &self.messages["None"][&active_index];
                                msg_header_cb(active_index, ui, events, msg_ref);
                            }
                        });
                    }
                });
            });
        });
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        if let Event::SelectMessage(id, index) = event {
            if *id == self.name {
                return self.set_active_message(*index)
            } else {
                return false
            }
        }

        if let Some(pm_vm) = self.vm_protobuf_message.as_mut() {
            if pm_vm.handle_event(event) {
                return true
            }
        }

        false
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}