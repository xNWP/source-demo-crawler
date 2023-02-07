use std::collections::BTreeMap;

use super::{ Event, ViewModel, Focusable, vm_demo_file::tick_to_time_string, table_constants, Filters };
use eframe::{egui::{self, Sense, CursorIcon, RichText, Layout}, emath::Align};
use source_demo_tool::demo_file::{FullGameEvent, FullGameEventKey, FullGameEventKeyType};
use egui_extras::{ TableBuilder, Column };

const EVENT_LIST_MIN_WIDTH: f32 = 360.0;
const EVENT_LIST_MAX_WIDTH: f32 = 500.0;
const DETAIL_LIST_MIN_WIDTH: f32 = 300.0;
const DETAIL_LIST_TYPE_WIDTH: f32 = 60.0;
const DETAIL_LIST_NAME_WIDTH: f32 = 120.0;

pub struct GameEventsToolViewModel {
    game_events: BTreeMap<
        String,
        BTreeMap<usize, FullGameEvent>
    >,
    display_events: Vec<(usize, FullGameEvent)>,
    active_index: Option<usize>,
    vm_active_keys: Option<GameEventKeysViewModel>,
    b_scroll_next: bool,
    tick_interval: f32,
    active_filter_index: usize,
    filterable_list: Vec<String>,
    filterable_data: Vec<(String, usize)>,
}

impl GameEventsToolViewModel {
    pub fn new(game_events_vec: Vec<FullGameEvent>, tick_interval: f32) -> Self {
        let mut filterable_data = BTreeMap::new();
        let mut game_events = BTreeMap::new();

        game_events.insert("None".to_owned(), BTreeMap::new());
        for i in 0..game_events_vec.len() {
            let ev = &game_events_vec[i];
            filterable_data
                .entry(ev.event_name.clone())
                .and_modify(|k| *k += 1)
                .or_insert(1 as usize);

            game_events
                .entry("None".to_owned())
                .and_modify(|m| {
                    m.insert(i, ev.clone());
                });

            game_events
                .entry(ev.event_name.clone())
                .and_modify(|m| {
                    m.insert(i, ev.clone());
                })
                .or_insert({
                    let mut m = BTreeMap::new();
                    m.insert(i, ev.clone());
                    m
                });
        }
        let filterable_data: Vec<(String, usize)> = filterable_data.into_iter().collect();

        let mut filterable_list = Vec::new();
        filterable_list.push("None".to_owned());
        for k in &filterable_data {
            filterable_list.push(
                format!(
                    "{} ({})",
                    k.0, k.1
                )
            );
        }

        let display_events
            = game_events["None".into()].clone().into_iter().collect();

        Self {
            game_events,
            tick_interval,
            filterable_list,
            filterable_data,
            display_events,
            active_index: None,
            vm_active_keys: None,
            b_scroll_next: true,
            active_filter_index: 0,
        }
    }

    fn set_active_index(&mut self, index: usize) -> bool {
        for ev in &self.display_events {
            if ev.0 == index {
                self.b_scroll_next = true;
                self.active_index = Some(index);

                let active_event = &ev.1;
                let keys = active_event.event_keys.clone();
                self.vm_active_keys = Some(
                    GameEventKeysViewModel::new(keys)
                );
                return true
            }
        }
        false
    }

    pub fn first_message(&mut self) -> bool {
        self.set_active_index(self.display_events.first().unwrap().0)
    }

    pub fn last_message(&mut self) -> bool {
        self.set_active_index(self.display_events.last().unwrap().0)
    }

    pub fn next_message(&mut self) -> bool {
        match self.active_index {
            Some(_) => {
                let mut b_capture_next = false;
                let mut next_index = None;
                for ev in &self.display_events {
                    if b_capture_next {
                        next_index = Some(ev.0);
                        break
                    }
                    if ev.0 == self.active_index.unwrap() {
                        b_capture_next = true
                    }
                }
                if let Some(i) = next_index {
                    self.set_active_index(i)
                } else {
                    false
                }
            },
            None => self.first_message()
        }
    }

    pub fn prev_message(&mut self) -> bool {
        match &self.active_index {
            Some(_) => {
                let mut prev_index = None;
                for ev in &self.display_events {
                    if ev.0 == self.active_index.unwrap() {
                        break
                    }
                    prev_index = Some(ev.0);
                }
                if let Some(i) = prev_index {
                    self.set_active_index(i)
                } else {
                    false
                }
            },
            None => self.first_message()
        }
    }
}

impl ViewModel for GameEventsToolViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>) {
        let avail_width = ui.available_width();
        let avail_height = ui.available_height();
        let table_height = avail_height - table_constants::BOTTOM_MARGIN;
        ui.set_width(avail_width);
        ui.set_height(avail_height);

        let event_list_width = {
            if self.active_index.is_some() {
                let rem_width = avail_width - DETAIL_LIST_MIN_WIDTH;
                if rem_width > EVENT_LIST_MAX_WIDTH {
                    EVENT_LIST_MAX_WIDTH
                } else if rem_width < EVENT_LIST_MIN_WIDTH {
                    EVENT_LIST_MIN_WIDTH
                } else {
                    rem_width
                }
            } else {
                avail_width
            }
        };

        // index
        // tick
        // time    =>   KeysList
        // name (id)
        egui::Grid::new("game_events_tool_grid").show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_width(event_list_width);
                ui.set_height(table_height);

                ui.with_layout(
                    Layout::right_to_left(Align::TOP),
                    |ui| {
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
                            self.display_events = {
                                if self.active_filter_index != 0 {
                                    let ev_str = &self.filterable_data[self.active_filter_index - 1].0;
                                    self.game_events[ev_str].clone().into_iter().collect()
                                } else {
                                    self.game_events["None".into()].clone().into_iter().collect()
                                }
                            };
                            self.first_message();
                        }
                    }
                );

                let mut table_builder = TableBuilder::new(ui);

                if self.b_scroll_next {
                    // get real index
                    let a_index = self.active_index.unwrap_or(0);
                    let mut real_index = 0;
                    for i in 0..self.display_events.len() {
                        let ev = &self.display_events[i];
                        if a_index == ev.0 {
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

                table_builder.striped(true)
                .column(Column::exact(table_constants::COL_INDEX_WIDTH))
                .column(Column::exact(table_constants::COL_TICK_WIDTH))
                .column(Column::exact(table_constants::COL_TIME_WIDTH))
                .column(Column::remainder())
                .header(table_constants::HEADER_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.label("Index");
                    });
                    row.col(|ui| {
                        ui.label("Tick");
                    });
                    row.col(|ui| {
                        ui.label("Time");
                    });
                    row.col(|ui| {
                        ui.label("Message (id)");
                    });
                })
                .body(|body| {
                    body.rows(
                        table_constants::ROW_HEIGHT,
                        self.display_events.len(),
                        |index, mut row| {
                            let game_event = &self.display_events[index];
                            let mut responses = Vec::new();

                            let real_index = game_event.0;
                            let is_active = match self.active_index {
                                Some(a_index) => a_index == real_index,
                                None => false
                            };

                            responses.push(row.col(|ui| {
                                let text = format!("{}", real_index);
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(table_constants::SELECTED_ITEM_COLOUR)
                                    );
                                } else {
                                    ui.label(text);
                                }
                            }).1);
                            responses.push(row.col(|ui| {
                                let text = format!("{}", game_event.1.event_tick);
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(table_constants::SELECTED_ITEM_COLOUR)
                                    );
                                } else {
                                    ui.label(text);
                                }
                            }).1);
                            responses.push(row.col(|ui| {
                                let text = tick_to_time_string(
                                    self.tick_interval,
                                    game_event.1.event_tick
                                );
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(table_constants::SELECTED_ITEM_COLOUR)
                                    );
                                } else {
                                    ui.label(text);
                                }
                            }).1);
                            responses.push(row.col(|ui| {
                                let text = format!(
                                    "{} ({})",
                                    game_event.1.event_name,
                                    game_event.1.event_id
                                );
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(table_constants::SELECTED_ITEM_COLOUR)
                                    );
                                } else {
                                    ui.label(text);
                                }
                            }).1);

                            for res in responses {
                                if res
                                .interact(Sense::click())
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked() {
                                    self.set_active_index(real_index);
                                    events.push(Event::SetFocus(Focusable::GameEventsList));
                                }
                            }
                        });
                });
            });

            if self.active_index.is_some() {
                let vm_detail = self.vm_active_keys.as_mut().unwrap();

                ui.vertical(|ui| {
                    ui.set_width(avail_width - event_list_width);
                    ui.set_height(table_height);

                    let event = &self.game_events["None".into()][&self.active_index.unwrap()];
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Frame: {}, Message: {}",
                            event.frame_index + 1,
                            event.message_index + 1
                        ));
                        if ui.button("Goto").clicked() {
                            events.append(&mut vec![
                                Event::SetTool("Frames"),
                                Event::SelectFrame("Frames", event.frame_index),
                                Event::SelectMessage("packet_data_messages", event.message_index)
                            ]);
                        }
                    });

                    ui.push_id(ui.next_auto_id(), |ui| {
                        vm_detail.draw(ui, events);
                    });
                });
            }
        });
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::SelectGameEvent(index) => {
                self.set_active_index(*index);
                true
            },
            Event::ClearFilter(filter) => {
                if let Filters::GameEvents = filter {
                    self.active_filter_index = 0;
                    self.display_events = self.game_events["None"].clone().into_iter().collect();
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

struct GameEventKeysViewModel {
    keys: Vec<FullGameEventKey>,
}

impl GameEventKeysViewModel {
    pub fn new(keys: Vec<FullGameEventKey>) -> Self {
        Self {
            keys
        }
    }
}

impl ViewModel for GameEventKeysViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, _events: &mut Vec<Event>) {
        // type
        // name
        // value
        egui_extras::TableBuilder::new(ui)
        .striped(true)
        .column(Column::exact(DETAIL_LIST_TYPE_WIDTH))
        .column(Column::initial(DETAIL_LIST_NAME_WIDTH).resizable(true))
        .column(Column::remainder())
        .header(table_constants::HEADER_HEIGHT, |mut row| {
            row.col(|ui| {
                ui.label("Type");
            });
            row.col(|ui| {
                ui.label("Name");
            });
            row.col(|ui| {
                ui.label("Value");
            });
        })
        .body(|body| {
            body.rows(
                table_constants::ROW_HEIGHT,
                self.keys.len(),
                |index, mut row| {
                    let key_ref = &self.keys[index];
                    row.col(|ui| {
                        ui.label(format!("{:?}", key_ref.key_type));
                    });
                    row.col(|ui| {
                        ui.label(key_ref.key_name.clone());
                    });
                    row.col(|ui| {
                        let text = match key_ref.key_type {
                            FullGameEventKeyType::Bool => format!("{}", key_ref.val_bool.unwrap()),
                            FullGameEventKeyType::Byte |
                            FullGameEventKeyType::Long |
                            FullGameEventKeyType::Short => format!("{}", key_ref.val_int.unwrap()),
                            FullGameEventKeyType::Float => format!("{}", key_ref.val_float.unwrap()),
                            FullGameEventKeyType::String => key_ref.val_string.as_ref().unwrap().clone(),
                        };
                        ui.label(text);
                    });
                });
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}