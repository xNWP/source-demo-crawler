use super::vm_demo_file::tick_to_time_string;
use super::{ Event, ViewModel, Focusable, TABLE_HEADER_HEIGHT, TABLE_BOTTOM_MARGIN };
use eframe::egui::{ self, RichText, Sense, CursorIcon };
use egui_extras::{ Column, TableBuilder };
use source_demo_tool::protobuf_message::ProtobufMessageEnumTraits;
use super::{ TABLE_ROW_HEIGHT, TABLE_SELECTED_ITEM_COLOUR };
use super::vm_protobuf_message::ProtobufMessageViewModel;
use source_demo_tool::demo_file::packet::MessageParseReturn;

const MESSAGE_INDEX_WIDTH: f32 = 60.0;
const MESSAGE_TICK_WIDTH: f32 = 80.0;
const MESSAGE_TIME_WIDTH: f32 = 110.0;
const MESSAGE_LIST_FULL_MAX_WIDTH: f32 = 500.0;
const MESSAGE_LIST_FULL_MIN_WIDTH: f32 = 420.0;
const MESSAGE_LIST_PARTIAL_MAX_WIDTH: f32 = MESSAGE_LIST_FULL_MAX_WIDTH - MESSAGE_TICK_WIDTH - MESSAGE_TIME_WIDTH;
const MESSAGE_LIST_PARTIAL_MIN_WIDTH: f32 = MESSAGE_LIST_FULL_MIN_WIDTH - MESSAGE_TICK_WIDTH - MESSAGE_TIME_WIDTH;
const MESSAGE_DETAIL_MIN_WIDTH: f32 = 420.0;

pub struct ProtobufMessageListViewModel<MessageType: ProtobufMessageEnumTraits> {
    pub vm_protobuf_message: Option<ProtobufMessageViewModel>,
    pub messages: Vec<MessageParseReturn<MessageType>>,
    active_message: Option<usize>,
    name: &'static str,
    b_scroll_next: bool,
    message_header_callback: Option<Box<
        dyn Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<MessageType>)
    >>,
    message_ticks: Option<Vec<i32>>,
    tick_interval: Option<f32>,
}

impl<MessageType: ProtobufMessageEnumTraits + Clone + 'static> ProtobufMessageListViewModel<MessageType> {
    pub fn new(name: &'static str, messages: Vec<MessageParseReturn<MessageType>>) -> Self {
        Self {
            name,
            messages,
            vm_protobuf_message: None,
            active_message: None,
            b_scroll_next: true,
            message_header_callback: None,
            message_ticks: None,
            tick_interval: None,
        }
    }

    pub fn set_tick_column(&mut self, ticks: Vec<i32>, tick_interval: f32) {
        self.message_ticks = Some(ticks);
        self.tick_interval = Some(tick_interval);
    }

    pub fn set_message_header_callback<F>(&mut self, callback: F)
    where F: Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<MessageType>) + 'static {
        self.message_header_callback = Some(Box::new(callback));
    }

    pub fn get_active_message(&self) -> &Option<usize> {
        &self.active_message
    }

    pub fn set_active_message(&mut self, index: usize) -> bool {
        if index >= self.messages.len() {
            return false
        }

        self.b_scroll_next = true;

        self.active_message = Some(index);
        let active_message = &self.messages[self.active_message.unwrap()].message;
        let active_message = active_message.as_ref().unwrap().clone();
        self.vm_protobuf_message = Some(
            ProtobufMessageViewModel::new(Box::new(active_message))
        );
        true
    }

    pub fn next_message(&mut self) -> bool {
        match &self.active_message {
            Some(index) => {
                self.set_active_message(index + 1)
            },
            None => {
                if self.messages.is_empty() {
                    false
                } else {
                    self.set_active_message(0);
                    true
                }
            }
        }
    }

    pub fn prev_message(&mut self) -> bool {
        match &self.active_message {
            Some(index) => {
                if *index == 0 {
                    false
                } else {
                    self.set_active_message(index - 1);
                    true
                }
            },
            None => {
                if self.messages.is_empty() {
                    false
                } else {
                    self.set_active_message(0);
                    true
                }
            }
        }
    }

    pub fn first_message(&mut self) {
        self.set_active_message(0);
    }

    pub fn last_message(&mut self) {
        self.set_active_message(self.messages.len() - 1);
    }
}

impl<MessageType: ProtobufMessageEnumTraits + ToString + Clone + 'static> ViewModel for ProtobufMessageListViewModel<MessageType> {
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
                    ui.set_height(avail_space.y - TABLE_BOTTOM_MARGIN);

                    let mut table_builder = TableBuilder::new(ui);

                    if self.b_scroll_next {
                        table_builder = table_builder.scroll_to_row
                            (self.active_message.unwrap_or(0), None);
                        self.b_scroll_next = false;
                    }

                    table_builder = table_builder.striped(true)
                    .column(Column::exact(MESSAGE_INDEX_WIDTH));

                    if self.message_ticks.is_some() {
                        table_builder = table_builder
                        .column(Column::exact(MESSAGE_TICK_WIDTH))
                        .column(Column::exact(MESSAGE_TIME_WIDTH));
                    }

                    table_builder.column(Column::remainder())
                    .header(TABLE_HEADER_HEIGHT, |mut row| {
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
                        body.rows(TABLE_ROW_HEIGHT, self.messages.len(), |index, mut row| {
                            let message_return = &self.messages[index];
                            let name = {
                                if message_return.message.is_some() {
                                    message_return.message.as_ref().unwrap().to_string()
                                } else {
                                    "????".into()
                                }
                            };

                            let is_active = {
                                if let Some(active_index) = self.active_message {
                                    if index == active_index {
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
                                let msg = format!("{}", index + 1);
                                if is_active {
                                    ui.label(RichText::new(msg).color(TABLE_SELECTED_ITEM_COLOUR));
                                } else {
                                    ui.label(msg);
                                }
                            }).1);

                            if let Some(ticks) = &self.message_ticks {
                                let tick = ticks[index];

                                responses.push(row.col(|ui| {
                                    let tick = tick.to_string();
                                    if is_active {
                                        ui.label(RichText::new(tick).color(TABLE_SELECTED_ITEM_COLOUR));
                                    } else {
                                        ui.label(tick);
                                    }
                                }).1);

                                responses.push(row.col(|ui| {
                                    let time = tick_to_time_string(self.tick_interval.unwrap(), tick);
                                    if is_active {
                                        ui.label(RichText::new(time).color(TABLE_SELECTED_ITEM_COLOUR));
                                    } else {
                                        ui.label(time);
                                    }
                                }).1);
                            };

                            responses.push(row.col(|ui| {
                                if is_active {
                                    ui.label(RichText::new(name).color(TABLE_SELECTED_ITEM_COLOUR));
                                } else {
                                    ui.label(name);
                                }
                            }).1);

                            for res in responses {
                                if res
                                .interact(Sense::click())
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked() {
                                    self.set_active_message(index);
                                    events.push(Event::SetFocus(Focusable::ProtobufMessageListViewModel(self.name)));
                                }
                            }
                        });
                    });
                });

                // TODO: FIXME: doesn't work
                /*
                ui.horizontal(|ui| {
                    ui.separator();
                    ui.set_height(avail_space.y);
                });
                */

                // message detail
                ui.vertical(|ui| {
                    ui.set_width(avail_space.x - message_list_width);
                    ui.set_height(avail_space.y - TABLE_BOTTOM_MARGIN);

                    if let Some(pm_vm) = self.vm_protobuf_message.as_mut() {
                        if let Some(msg_header_cb) = &mut self.message_header_callback {
                            let active_index = self.active_message.unwrap();
                            let msg_ref = &self.messages[active_index];
                            msg_header_cb(active_index, ui, events, msg_ref);
                        }

                        pm_vm.draw(ui, events);
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