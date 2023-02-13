use std::{path::PathBuf, any::Any};

use eframe::egui;

// view models: contains our domain data, additionally state
//     logic for rendering each component, the view is the draw function
//     which should minimize computation and focus on drawing the elements.
mod vm_no_files_open;
mod vm_opening_files;
mod vm_main;
mod vm_demo_file;
mod vm_header_tool;
mod vm_frames_tool;
mod vm_protobuf_message;
mod vm_packet_data;
mod vm_user_messages_tool;
mod vm_protobuf_message_list;
mod vm_server_info_tool;
mod vm_game_events_tool;
mod vm_abouthelp;
mod vm_data_tables;
mod vm_tasks_tool;
// widgets: small tools for displaying common gui components.
mod w_copyable_field;
// widget functions: small tools for displaying common gui components implemented as functions.
mod wfn_text_edit_singleline;

use vm_main::MainViewModel;

pub struct NewCrawlerApp {
    main_view_model: MainViewModel,
    events: Vec<Event>,
    frame_counter: usize,
    is_first_run: bool,
}

impl NewCrawlerApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        NewCrawlerApp {
            main_view_model: MainViewModel::new(),
            events: Vec::new(),
            frame_counter: 1,
            is_first_run: true,
        }
    }
}

impl eframe::App for NewCrawlerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let title = format!(
            "Source Demo Crawler v{}",
            VERSION
        );
        if self.is_first_run {
            frame.set_window_title(title.as_str());
            self.is_first_run = false;
        }


        let mut unhandled_events = Vec::new();
        let mut event_iter = self.events.iter();
        loop {
            match event_iter.next() {
                Some(event) => {
                    if let Event::NewFile(filepath) = event {
                        let title = format!(
                            "Source Demo Crawler v{} -- {}",
                            VERSION,
                            filepath.file_stem().unwrap().to_str().unwrap()
                        );
                        frame.set_window_title(title.as_str());
                        continue
                    }

                    if !self.main_view_model.handle_event(event) {
                        unhandled_events.push(event);
                    }
                },
                None => break
            }
        }

        if !unhandled_events.is_empty() {
            eprintln!("Unhandled events on frame: {}", self.frame_counter);
            for event in unhandled_events {
                eprintln!("Event: {:?}", event);
            }
        }

        self.events.clear();

        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_view_model.draw(ui, &mut self.events);
        });

        self.frame_counter += 1;
    }

    fn persist_egui_memory(&self) -> bool { false }
    fn persist_native_window(&self) -> bool { false }
}

#[derive(Debug, Clone)]
pub enum Focusable {
    None,
    FramesListViewModel,
    ProtobufMessageListViewModel(&'static str),
    GameEventsList,
    SendTables,
}

#[derive(Debug)]
pub enum Filters {
    UserMessages,
    GameEvents,
}

#[derive(Debug)]
pub enum Event {
    BeginOpenFile,
    NewFile(PathBuf),
    SetFocus(Focusable),
    SelectFrame(&'static str, usize),
    SelectMessage(&'static str, usize),
    SetTool(&'static str),
    SelectGameEvent(usize),
    ClearFilter(Filters),
    EmitNetMsgWarnErrs,
    EmitUserMsgWarnErrs,
}

/*
impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Event::BeginOpenFile => f.write_str("BeginOpenFile"),
            Event::NewFile(filepath) => f.write_fmt(format_args!("NewFile({})", filepath.to_str().unwrap())),
            Event::SetFocus(foc) => f.write_fmt(format_args!("SetFocus({:?})", foc)),
            Event::SelectFrame(tool_name, index) => f.write_fmt(format_args!("SelectFrame({}, {})", tool_name, index)),
            Event::SelectMessage(tool, index) => f.write_fmt(format_args!("SelectMessage({}, {})", tool, index)),
            Event::SetTool(tool_name) => f.write_fmt(format_args!("SetTool({})", tool_name)),
            Event::SelectGameEvent(index) => f.write_fmt(format_args!("SelectGameEvent({})", index)),
            Event::ClearFilter(filt) => f.write_fmt(format_args!("ClearFilter({:?})", filt)),
            Event::EmitNetMsgWarnErrs => f.write_str("EmitNetMsgWarnErrs"),
        }
    }
}
*/

pub trait ViewModel: Send {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>);
    fn handle_event(&mut self, _event: &Event) -> bool { false }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub mod table_constants {
    pub const COL_TICK_WIDTH: f32 = 80.0;
    pub const COL_INDEX_WIDTH: f32 = 70.0;
    pub const COL_TIME_WIDTH: f32 = 120.0;
    pub const HEADER_HEIGHT: f32 = 20.0;
    pub const ROW_HEIGHT: f32 = 18.0;
    pub const SELECTED_ITEM_COLOUR: eframe::egui::Color32 = eframe::egui::Color32::LIGHT_YELLOW;
    pub const BOTTOM_MARGIN: f32 = 5.0;
}