use super::{ Event, ViewModel, TABLE_HEADER_HEIGHT, TABLE_ROW_HEIGHT, TABLE_BOTTOM_MARGIN, TABLE_SELECTED_ITEM_COLOUR, Focusable, vm_demo_file::tick_to_time_string };
use eframe::egui::{self, Sense, CursorIcon, RichText};
use source_demo_tool::demo_file::{FullGameEvent, FullGameEventKey, FullGameEventKeyType};
use egui_extras::{ TableBuilder, Column };

const EVENT_LIST_INDEX_WIDTH: f32 = 60.0;
const EVENT_LIST_TICK_WIDTH: f32 = 80.0;
const EVENT_LIST_TIME_WIDTH: f32 = 110.0;
const EVENT_LIST_MIN_WIDTH: f32 = 360.0;
const EVENT_LIST_MAX_WIDTH: f32 = 500.0;
const DETAIL_LIST_MIN_WIDTH: f32 = 300.0;
const DETAIL_LIST_TYPE_WIDTH: f32 = 60.0;
const DETAIL_LIST_NAME_WIDTH: f32 = 120.0;

pub struct GameEventsToolViewModel {
    game_events: Vec<FullGameEvent>,
    active_index: Option<usize>,
    vm_active_keys: Option<GameEventKeysViewModel>,
    b_scroll_next: bool,
    tick_interval: f32,
}

impl GameEventsToolViewModel {
    pub fn new(game_events: Vec<FullGameEvent>, tick_interval: f32) -> Self {
        Self {
            game_events,
            tick_interval,
            active_index: None,
            vm_active_keys: None,
            b_scroll_next: true,
        }
    }

    fn set_active_index(&mut self, index: usize) -> bool {
        if index >= self.game_events.len() {
            return false
        }

        self.active_index = Some(index);

        let keys = self.game_events[index].event_keys.clone();
        self.vm_active_keys = Some(
            GameEventKeysViewModel::new(keys)
        );

        self.b_scroll_next = true;
        true
    }

    pub fn first_message(&mut self) -> bool {
        self.set_active_index(0)
    }

    pub fn last_message(&mut self) -> bool {
        let mut index = self.game_events.len();
        if index != 0 {
            index -= 1;
        }
        self.set_active_index(index)
    }

    pub fn next_message(&mut self) -> bool {
        let index = match self.active_index {
            Some(i) => i + 1,
            None => 0
        };
        self.set_active_index(index)
    }

    pub fn prev_message(&mut self) -> bool {
        let index = match self.active_index {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            },
            None => 0
        };
        self.set_active_index(index)
    }
}

impl ViewModel for GameEventsToolViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>) {
        let avail_width = ui.available_width();
        let avail_height = ui.available_height();
        let table_height = avail_height - TABLE_BOTTOM_MARGIN;
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

                let mut table_builder = TableBuilder::new(ui);

                if self.b_scroll_next {
                    self.b_scroll_next = false;
                    table_builder = table_builder.scroll_to_row(self.active_index.unwrap_or(0), None);
                }

                table_builder.striped(true)
                .column(Column::exact(EVENT_LIST_INDEX_WIDTH))
                .column(Column::exact(EVENT_LIST_TICK_WIDTH))
                .column(Column::exact(EVENT_LIST_TIME_WIDTH))
                .column(Column::remainder())
                .header(TABLE_HEADER_HEIGHT, |mut row| {
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
                        TABLE_ROW_HEIGHT,
                        self.game_events.len(),
                        |index, mut row| {
                            let game_event = &self.game_events[index];
                            let mut responses = Vec::new();
                            let is_active = match self.active_index {
                                Some(a_index) => a_index == index,
                                None => false
                            };

                            responses.push(row.col(|ui| {
                                let text = format!("{}", index);
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(TABLE_SELECTED_ITEM_COLOUR)
                                    );
                                } else {
                                    ui.label(text);
                                }
                            }).1);
                            responses.push(row.col(|ui| {
                                let text = format!("{}", game_event.event_tick);
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(TABLE_SELECTED_ITEM_COLOUR)
                                    );
                                } else {
                                    ui.label(text);
                                }
                            }).1);
                            responses.push(row.col(|ui| {
                                let text = tick_to_time_string(
                                    self.tick_interval,
                                    game_event.event_tick
                                );
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(TABLE_SELECTED_ITEM_COLOUR)
                                    );
                                } else {
                                    ui.label(text);
                                }
                            }).1);
                            responses.push(row.col(|ui| {
                                let text = format!(
                                    "{} ({})",
                                    game_event.event_name,
                                    game_event.event_id
                                );
                                if is_active {
                                    ui.label(
                                        RichText::new(text)
                                        .color(TABLE_SELECTED_ITEM_COLOUR)
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
                                    self.set_active_index(index);
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

                    ui.push_id(ui.next_auto_id(), |ui| {
                        vm_detail.draw(ui, events);
                    });
                });
            }
        });
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
        .header(TABLE_HEADER_HEIGHT, |mut row| {
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
                TABLE_ROW_HEIGHT,
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