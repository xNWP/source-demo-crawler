use super::{ViewModel, wfn_text_edit_singleline::wfn_text_edit_singleline};

use source_demo_tool::protobuf_message::ProtobufMessageEnumTraits;

use egui_extras::{ TableBuilder, Column };

use source_demo_tool::demo_file::packet::protobuf_value::ProtobufValue;

const FIELD_NAME_WIDTH: f32 = 200.0;

pub struct ProtobufMessageViewModel {
    pub message: Box<dyn ProtobufMessageEnumTraits>,
    pub field_data: Vec<(String, ProtobufValue)>,
    hide_none_values: bool,
}

impl ProtobufMessageViewModel {
    pub fn new(message: Box<dyn ProtobufMessageEnumTraits>) -> Self {
        let mut rval = Self {
            message,
            field_data: Vec::new(),
            hide_none_values: false,
        };
        rval.update_field_data();

        rval
    }

    pub fn hide_none_values_set(&mut self, value: bool) {
        self.hide_none_values = value;
        self.update_field_data();
    }

    pub fn hide_none_values_get(&self) -> bool {
        self.hide_none_values
    }

    fn update_field_data(&mut self) {
        let mut int_field_data: Vec<(String, ProtobufValue)> = self.message
            .to_vec()
            .into_iter()
            .map(|(name, val)| {
              (name.to_string(), val)
            })
            .collect();

        // flatten the field_data and skip None values if requested
        let mut field_data;
        loop {
            let mut did_work = false;
            field_data = Vec::new();

            for (field_name, field_val) in int_field_data {
                match field_val {
                    ProtobufValue::Proto(vec_proto_fields) => {
                        for (sub_field_name, sub_field_value) in vec_proto_fields {
                            let sub_name = format!("{}.{}", field_name, sub_field_name);
                            field_data.push((sub_name, sub_field_value));
                        }
                        did_work = true;
                    },
                    ProtobufValue::Repeated(vec_proto_values) => {
                        let mut it = 0;
                        for sub_value in vec_proto_values {
                            let sub_name = format!("{}[{}]", field_name, it);
                            field_data.push((sub_name, sub_value));
                            it += 1;
                        }
                        did_work = true;
                    },
                    ProtobufValue::None => {
                        if !self.hide_none_values {
                            field_data.push((field_name, ProtobufValue::None));
                        }
                    },
                    val => field_data.push((field_name, val))
                }
            }

            if !did_work {
                break
            }

            // did work, re-run the loop
            int_field_data = field_data;
        }

        self.field_data = field_data;
    }
}

impl ViewModel for ProtobufMessageViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, _events: &mut Vec<super::Event>) {
        ui.push_id(3, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());

            if ui.checkbox(&mut self.hide_none_values, "Hide None Values").changed() {
                self.update_field_data();
            }

            TableBuilder::new(ui)
            .column(Column::initial(FIELD_NAME_WIDTH).resizable(true))
            .column(Column::remainder())
            .body(|body| {
                body.rows(super::table_constants::ROW_HEIGHT, self.field_data.len(), |index, mut row| {
                    let field = &self.field_data[index];
                    row.col(|ui| {
                        ui.label(&field.0);
                    });
                    row.col(|ui| {
                        let mut val_str: String = match &field.1 {
                            ProtobufValue::None => "None".to_string(),
                            ProtobufValue::VarInt(v) => v.to_string(),
                            ProtobufValue::Length(v) => format!("Data ({} Bytes)", v.len()),
                            ProtobufValue::String(v) => v.clone(),
                            ProtobufValue::Fixed32(v) => v.to_string(),
                            ProtobufValue::Float32(v) => v.to_string(),
                            ProtobufValue::Proto(_v) => panic!("Proto should be flattened out"),
                            ProtobufValue::Repeated(_v) => panic!("Repeated should be flattened out"),
                        };

                        match &field.1 {
                            ProtobufValue::Length(_) => { ui.label(val_str); },
                            _ => { wfn_text_edit_singleline(ui, &mut val_str, None, true); }
                        }
                    });
                });
            });
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}