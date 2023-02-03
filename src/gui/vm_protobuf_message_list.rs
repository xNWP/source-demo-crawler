use super::{ Event, ViewModel, Focusable };
use eframe::egui::{ self, RichText, Sense, CursorIcon };
use egui_extras::{ Column, TableBuilder };
use source_demo_tool::protobuf_message::ProtobufMessageEnumTraits;
use super::{ TABLE_ROW_HEIGHT, SELECTED_ITEM_COLOUR };
use super::vm_protobuf_message::ProtobufMessageViewModel;
use source_demo_tool::demo_file::packet::MessageParseReturn;

pub struct ProtobufMessageListViewModel<MessageType: ProtobufMessageEnumTraits> {
    pub vm_protobuf_message: Option<ProtobufMessageViewModel>,
    pub messages: Vec<MessageParseReturn<MessageType>>,
    active_message: Option<usize>,
    name: &'static str,
    b_scroll_next: bool,
    message_header_callback: Option<Box<
        dyn Fn(usize, &mut egui::Ui, &mut Vec<Event>, &MessageParseReturn<MessageType>)
    >>,
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
        }
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
                let message_list_width = 200.0;
                let message_detail_width = avail_space.x - message_list_width;

                // message list
                ui.vertical(|ui| {
                    ui.set_width(message_list_width);
                    ui.set_height(avail_space.y);

                    let mut table_builder = TableBuilder::new(ui);

                    if self.b_scroll_next {
                        table_builder = table_builder.scroll_to_row
                            (self.active_message.unwrap_or(0), None);
                        self.b_scroll_next = false;
                    }

                    table_builder.striped(true)
                    .column(Column::exact(message_list_width))
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

                            let res = row.col(|ui| {
                                if is_active {
                                    ui.label(RichText::new(name).color(SELECTED_ITEM_COLOUR));
                                } else {
                                    ui.label(name);
                                }
                            }).1.interact(Sense::click());

                            if res.clicked() {
                                self.set_active_message(index);
                                events.push(Event::SetFocus(Focusable::ProtobufMessageListViewModel(self.name)));
                            }

                            res.on_hover_cursor(CursorIcon::PointingHand);
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
                    ui.set_width(message_detail_width);
                    ui.set_height(avail_space.y);

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