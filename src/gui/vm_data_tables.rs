use source_demo_tool::demo_file::frame::DataTablesData;

use super::{ Event, ViewModel, table_constants::{HEADER_HEIGHT, ROW_HEIGHT, SELECTED_ITEM_COLOUR}, Focusable, wfn_text_edit_singleline::wfn_text_edit_singleline };
use egui_extras::{ TableBuilder, Column };
use eframe::{egui::{self, Sense, RichText, CursorIcon, Layout}, emath::Align};

const CLASS_DESC_CLASS_ID_WIDTH: f32 = 80.0;
const CLASS_DESC_TABLE_NAME_WIDTH: f32 = 280.0;
const SEND_TABLE_IS_END_WIDTH: f32 = 50.0;
const SEND_TABLE_NEEDS_DECODER_WIDTH: f32 = 100.0;

const SEND_PROP_NAME_WIDTH: f32 = 160.0;
const SEND_PROP_TYPE_WIDTH: f32 = 30.0;
const SEND_PROP_DT_NAME_WIDTH: f32 = 160.0;
const SEND_PROP_FLAGS_WIDTH: f32 = 160.0;
const SEND_PROP_HVAL_WIDTH: f32 = 100.0;
const SEND_PROP_LVAL_WIDTH: f32 = 100.0;
const SEND_PROP_NBITS_WIDTH: f32 = 35.0;
const SEND_PROP_NELEMS_WIDTH: f32 = 35.0;

pub struct DataTablesViewModel {
    data_tables: DataTablesData,
    active_send_table_index: Option<usize>,
    b_send_table_scroll_next: bool,
    pub active_mode: &'static str,
}

impl DataTablesViewModel {
    pub fn new(data_tables: DataTablesData) -> Self {
        Self {
            data_tables,
            active_send_table_index: None,
            b_send_table_scroll_next: true,
            active_mode: "Class Descriptions",
        }
    }

    fn set_active_send_table(&mut self, index: usize) -> bool {
        if index >= self.data_tables.send_tables.len() {
            return false
        }
        self.b_send_table_scroll_next = true;
        self.active_send_table_index = Some(index);
        return true
    }

    pub fn send_table_next(&mut self) -> bool {
        let index = match self.active_send_table_index {
            Some(k) => k + 1,
            None => 0
        };
        self.set_active_send_table(index)
    }

    pub fn send_table_prev(&mut self) -> bool {
        let index = match self.active_send_table_index {
            Some(k) => k - 1,
            None => 0
        };
        self.set_active_send_table(index)
    }

    pub fn send_table_first(&mut self) -> bool {
        self.set_active_send_table(0)
    }

    pub fn send_table_last(&mut self) -> bool {
        let mut index = self.data_tables.send_tables.len();
        if index != 0 {
            index -= 1;
        }
        self.set_active_send_table(index)
    }
}

impl ViewModel for DataTablesViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<Event>) {
        let avail_space = ui.available_size();

        ui.with_layout(
            Layout::right_to_left(Align::TOP),
            |ui| {
                ui.add_space(15.0);
                ui.radio_value(
                    &mut self.active_mode,
                    "Send Tables",
                    "Send Tables"
                );
                ui.radio_value(
                    &mut self.active_mode,
                    "Class Descriptions",
                    "Class Descriptions"
                );
            }
        );
        ui.separator();

        if self.active_mode == "Class Descriptions" {
            // class descriptions
            // - class id
            // - table name
            // - network name
            ui.vertical(|ui| {
                ui.set_width(avail_space.x);
                let height = ui.available_height();
                ui.set_height(height);
                ui.heading("Class Descriptions");

                let class_descs = &self.data_tables.class_descriptions;
                TableBuilder::new(ui)
                .striped(true)
                .column(Column::exact(CLASS_DESC_CLASS_ID_WIDTH))
                .column(Column::exact(CLASS_DESC_TABLE_NAME_WIDTH))
                .column(Column::remainder())
                .header(HEADER_HEIGHT, |mut row| {
                    row.col(|ui| {
                        ui.label("Class ID");
                    });
                    row.col(|ui| {
                        ui.label("Table Name");
                    });
                    row.col(|ui| {
                        ui.label("Network Name");
                    });
                })
                .body(|body| {
                    body.rows(
                        ROW_HEIGHT,
                        class_descs.len(),
                        |index, mut row| {
                            row.col(|ui| {
                                let text = format!("{}", class_descs[index].class_id);
                                ui.label(text);
                            });
                            row.col(|ui| {
                                let mut text = class_descs[index].table_name.clone();
                                wfn_text_edit_singleline(ui, &mut text, None, false);
                            });
                            row.col(|ui| {
                                let mut text = class_descs[index].network_name.clone();
                                wfn_text_edit_singleline(ui, &mut text, None, false);
                            });
                        }
                    );
                });
            });
        } else if self.active_mode == "Send Tables" {
            // send tables
            // - is end
            // - needs decoder
            // - net table name
            ui.vertical(|ui| {
                ui.set_width(avail_space.x);
                let height = ui.available_height();
                ui.set_height(height);
                ui.heading("Send Tables");
                let height = ui.available_height();

                egui::Grid::new(ui.next_auto_id())
                .show(ui, |ui| {
                    // Send Table List
                    ui.vertical(|ui| {
                        ui.set_width(avail_space.x);
                        ui.set_height({
                            if self.active_send_table_index.is_some() {
                                height / 2.0
                            } else {
                                height
                            }
                        });

                        let mut table_builder = TableBuilder::new(ui);

                        if self.b_send_table_scroll_next {
                            table_builder = table_builder.scroll_to_row(
                                self.active_send_table_index.unwrap_or(0),
                                None
                            );
                            self.b_send_table_scroll_next = false;
                        }

                        table_builder
                        //.striped(true)
                        .column(Column::exact(SEND_TABLE_IS_END_WIDTH))
                        .column(Column::exact(SEND_TABLE_NEEDS_DECODER_WIDTH))
                        .column(Column::remainder())
                        .header(HEADER_HEIGHT, |mut row| {
                            row.col(|ui| {
                                ui.label("Is End");
                            });
                            row.col(|ui| {
                                ui.label("Needs Decoder");
                            });
                            row.col(|ui| {
                                ui.label("Net Table Name");
                            });
                        })
                        .body(|body| {
                            body.rows(
                                ROW_HEIGHT,
                                self.data_tables.send_tables.len(),
                                |index, mut row| {
                                    let send_tables = &self.data_tables.send_tables;
                                    let st = &send_tables[index];
                                    let mut responses = Vec::new();

                                    let b_is_active = match self.active_send_table_index {
                                        Some(i) => i == index,
                                        None => false
                                    };

                                    responses.push(row.col(|ui| {
                                        let text = match st.is_end {
                                            Some(x) => format!("{}", x),
                                            None => "None".to_owned()
                                        };
                                        if b_is_active {
                                            ui.label(RichText::new(text).color(SELECTED_ITEM_COLOUR));
                                        } else {
                                            ui.label(text);
                                        }
                                    }).1);
                                    responses.push(row.col(|ui| {
                                        let text = match st.needs_decoder {
                                            Some(x) => format!("{}", x),
                                            None => "None".to_owned()
                                        };
                                        if b_is_active {
                                            ui.label(RichText::new(text).color(SELECTED_ITEM_COLOUR));
                                        } else {
                                            ui.label(text);
                                        }
                                    }).1);
                                    let tmp_res = row.col(|ui| {
                                        let mut text = match &st.net_table_name {
                                            Some(x) => x.clone(),
                                            None => "None".to_owned()
                                        };
                                        if b_is_active {
                                            wfn_text_edit_singleline(ui, &mut text, Some(SELECTED_ITEM_COLOUR), false);
                                        } else {
                                            wfn_text_edit_singleline(ui, &mut text, None, false);
                                        }
                                    }).1;
                                    responses.push(tmp_res);

                                    for res in responses {
                                        if res
                                            .interact(Sense::click())
                                            .on_hover_cursor(CursorIcon::PointingHand)
                                            .clicked() {
                                                self.set_active_send_table(index);
                                                events.push(Event::SetFocus(Focusable::SendTables));
                                            }
                                    }
                                }
                            )
                        });
                    });

                    // send table detail
                    if let Some(active_index) = self.active_send_table_index {
                        ui.end_row();
                        ui.push_id(ui.next_auto_id(), |ui| {
                            ui.vertical(|ui| {
                                ui.set_width(avail_space.x);
                                ui.set_height(height / 2.0);
                                ui.heading("Properties/Fields");

                                let active_st = &self.data_tables.send_tables[active_index];
                                let send_props = &active_st.SendProp;

                                // Send Prop
                                // - name
                                // - type
                                // - DataTable name
                                // - flags
                                // - high value
                                // - low value
                                // - num bits
                                // - num elements
                                // - priority
                                TableBuilder::new(ui)
                                //.striped(true)
                                .column(Column::initial(SEND_PROP_NAME_WIDTH).resizable(true))
                                .column(Column::initial(SEND_PROP_TYPE_WIDTH).resizable(true))
                                .column(Column::initial(SEND_PROP_DT_NAME_WIDTH).resizable(true))
                                .column(Column::initial(SEND_PROP_FLAGS_WIDTH).resizable(true))
                                .column(Column::initial(SEND_PROP_HVAL_WIDTH).resizable(true))
                                .column(Column::initial(SEND_PROP_LVAL_WIDTH).resizable(true))
                                .column(Column::initial(SEND_PROP_NBITS_WIDTH).resizable(true))
                                .column(Column::initial(SEND_PROP_NELEMS_WIDTH).resizable(true))
                                .column(Column::remainder())
                                .header(HEADER_HEIGHT, |mut row| {
                                    row.col(|ui| {
                                        ui.label("Name");
                                    });
                                    row.col(|ui| {
                                        ui.label("Type");
                                    });
                                    row.col(|ui| {
                                        ui.label("DataTable Name");
                                    });
                                    row.col(|ui| {
                                        ui.label("Flags");
                                    });
                                    row.col(|ui| {
                                        ui.label("Hi-Value");
                                    });
                                    row.col(|ui| {
                                        ui.label("Lo-Value");
                                    });
                                    row.col(|ui| {
                                        ui.label("# Bits");
                                    });
                                    row.col(|ui| {
                                        ui.label("# Elems");
                                    });
                                    row.col(|ui| {
                                        ui.label("Priority");
                                    });
                                })
                                .body(|body| {
                                    body.rows(
                                        ROW_HEIGHT,
                                        send_props.len(),
                                        |index, mut row| {
                                            let field = &send_props[index];

                                            row.col(|ui| {
                                                let mut text = match &field.var_name {
                                                    Some(s) => s.clone(),
                                                    None => "None".to_owned()
                                                };
                                                wfn_text_edit_singleline(ui, &mut text, None, false);
                                            });
                                            row.col(|ui| {
                                                ui.label(match &field.sendprop_type {
                                                    Some(x) => format!("{}", x),
                                                    None => "None".to_owned()
                                                });
                                            });
                                            row.col(|ui| {
                                                let mut text = match &field.dt_name {
                                                    Some(s) => s.clone(),
                                                    None => "None".to_owned()
                                                };
                                                wfn_text_edit_singleline(ui, &mut text, None, false);
                                            });
                                            row.col(|ui| {
                                                let mut text = match &field.flags {
                                                    Some(x) => format!("0x{:0>16x}", x),
                                                    None => "None".to_owned()
                                                };
                                                wfn_text_edit_singleline(ui, &mut text, None, false);
                                            });
                                            row.col(|ui| {
                                                let mut text = match &field.high_value {
                                                    Some(x) => format!("0x{:0>8x}", x),
                                                    None => "None".to_owned()
                                                };
                                                wfn_text_edit_singleline(ui, &mut text, None, false);
                                            });
                                            row.col(|ui| {
                                                let mut text = match &field.low_value {
                                                    Some(x) => format!("0x{:0>8x}", x),
                                                    None => "None".to_owned()
                                                };
                                                wfn_text_edit_singleline(ui, &mut text, None, false);
                                            });
                                            row.col(|ui| {
                                                ui.label(match &field.num_bits {
                                                    Some(x) => format!("{}", x),
                                                    None => "None".to_owned()
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.label(match &field.num_elements {
                                                    Some(x) => format!("{}", x),
                                                    None => "None".to_owned()
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.label(match &field.priority {
                                                    Some(x) => format!("{}", x),
                                                    None => "None".to_owned()
                                                });
                                            });
                                        }
                                    );
                                });
                            });
                        });
                    }
                });
            });
        } else {
            ui.heading("Bad UI State, please report this.");
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}