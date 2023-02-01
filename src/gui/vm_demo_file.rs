use super::{
    ViewModel,
    Event,
    vm_header_tool::HeaderToolViewModel,
    vm_frames_tool::FramesToolViewModel,
    vm_user_messages_tool::UserMessagesToolViewModel, Focusable,
};
use source_demo_tool::demo_file::DemoFile;
use eframe::{
    egui::{
        self,
        Color32,
        CursorIcon,
        FontFamily,
        FontId,
        RichText,
        Sense,
        style::Margin,
    },
    epaint::{ Rounding, Stroke },
};

const TOOL_LABEL_SIZE: f32 = 14.0;
const TOOL_TOPBOTTOM_MARGIN: f32 = TOOL_LABEL_SIZE / 4.0;
const TOOL_LEFTRIGHT_MARGIN: f32 = TOOL_LABEL_SIZE;
const TOOL_BACKGROUND_COLOUR: Color32 = Color32::from_rgb(34, 34, 34);
const TOOL_LABEL_COLOUR: Color32 = Color32::from_rgb(242, 242, 242);
const TOOL_STROKE_COLOUR: Color32 = Color32::from_rgb(255, 220, 255);
const TOOL_ACTIVE_HOVER_COLOUR: Color32 = Color32::from_rgb(60, 60, 60);
const TOOL_STROKE_WIDTH: f32 = 1.0;
const TOOL_ROUNDING_RADIUS: f32 = 4.0;

pub struct DemoFileTools {
    name: &'static str,
    vm: Box<dyn ViewModel>,
    focus: Focusable,
}

pub struct DemoFileViewModel {
    pub demo_file: DemoFile,
    tools: Vec<DemoFileTools>,
    active_tool_index: usize,
    hover_tool_index: Option<usize>,
    inner_events: Vec<Event>,
    b_inner_events_sent_last: bool,
}

impl DemoFileViewModel {
    pub fn new(demo_file: DemoFile) -> Self {
        let header = demo_file.header.clone();
        let tick_interval = match demo_file.get_server_info() {
            Some(si) => si.tick_interval.unwrap_or(0.0),
            None => 0.0
        };
        let frames = demo_file.frames.clone();
        let user_messages = demo_file.get_user_messages();
        let tools: Vec<DemoFileTools> = vec![
            DemoFileTools {
                name: "Header",
                vm: Box::new(HeaderToolViewModel::new(header)),
                focus: Focusable::None,
            },
            DemoFileTools {
                name: "Frames",
                vm: Box::new(FramesToolViewModel::new(frames, tick_interval)),
                focus: Focusable::FramesListViewModel,
            },
            DemoFileTools {
                name: "User Messages",
                vm: Box::new(UserMessagesToolViewModel::new(user_messages)),
                focus: Focusable::ProtobufMessageListViewModel("user_messages"),
            },
        ];

        Self {
            demo_file,
            tools,
            active_tool_index: 0, // header tool
            hover_tool_index: None,
            inner_events: Vec::new(),
            b_inner_events_sent_last: false,
        }
    }

    pub fn get_active_tool(&mut self) -> &mut dyn ViewModel {
        &mut *self.tools[self.active_tool_index].vm
    }

    pub fn set_active_tool(&mut self, index: usize) -> bool {
        if index >= self.tools.len() {
            false
        } else {
            self.active_tool_index = index;
            self.inner_events
                .push(Event::SetFocus(
                    self.tools[self.active_tool_index].focus.clone()
                ));
            true
        }
    }

    pub fn set_active_tool_by_name(&mut self, name: &'static str) -> bool {
        for i in 0..self.tools.len() {
            let tool = &self.tools[i];
            if tool.name == name {
                return self.set_active_tool(i)
            }
        }
        return false
    }

    pub fn next_tool(&mut self) -> bool {
        self.set_active_tool(self.active_tool_index + 1)
    }

    pub fn prev_tool(&mut self) -> bool {
        self.set_active_tool(self.active_tool_index - 1)
    }

    pub fn first_tool(&mut self) {
        self.set_active_tool(0);
    }

    pub fn last_tool(&mut self) {
        self.set_active_tool(self.tools.len() - 1);
    }
}

impl ViewModel for DemoFileViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, events: &mut Vec<super::Event>) {
        // handle inner events
        if self.b_inner_events_sent_last {
            self.b_inner_events_sent_last = false;
        } else {
            events.append(&mut self.inner_events);
        }

        // draw tool buttons
        ui.horizontal(|ui| {
            for i in 0..self.tools.len() {
                let tool = &self.tools[i];

                let mut bg_colour = {
                    if i == self.active_tool_index {
                        TOOL_ACTIVE_HOVER_COLOUR
                    } else {
                        TOOL_BACKGROUND_COLOUR
                    }
                };

                if let Some(hover_index) = self.hover_tool_index {
                    if i == hover_index {
                        bg_colour = TOOL_ACTIVE_HOVER_COLOUR;
                    }
                }

                let tool_res = egui::Frame::none()
                    .inner_margin(Margin::symmetric(
                        TOOL_LEFTRIGHT_MARGIN,
                        TOOL_TOPBOTTOM_MARGIN
                    ))
                    .fill(bg_colour)
                    .stroke(Stroke::new(TOOL_STROKE_WIDTH, TOOL_STROKE_COLOUR))
                    .rounding(Rounding::same(TOOL_ROUNDING_RADIUS))
                    .show(ui, |ui| {
                        ui.label(RichText::new(tool.name)
                            .font(FontId { size: TOOL_LABEL_SIZE, family: FontFamily::Proportional })
                            .color(TOOL_LABEL_COLOUR));
                    }).response.interact(Sense::click());

                if tool_res.hovered() {
                    if self.hover_tool_index.is_none() { // don't double send event
                        self.hover_tool_index = Some(i);
                    }
                } else {
                    if let Some(hover_index) = self.hover_tool_index {
                        if i == hover_index {
                            self.hover_tool_index = None;
                        }
                    }
                }
                if tool_res.clicked() {
                    self.set_active_tool(i);
                }

                tool_res.on_hover_cursor(CursorIcon::PointingHand);
            }
        });

        // draw tool
        ui.separator();

        egui::Frame::none().show(ui, |ui| {
            self.tools[self.active_tool_index].vm.draw(ui, events);
        });
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        if let Event::SetFocus(focusable) = event {
            self.tools[self.active_tool_index].focus = focusable.clone();
            return true
        }

        if let Event::SetTool(tool_name) = event {
            return self.set_active_tool_by_name(tool_name)
        }

        for tool in &mut self.tools {
            if tool.vm.handle_event(event) {
                return true
            }
        }
        false
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}