use super::{
    Event, ViewModel, Focusable,
    vm_demo_file::DemoFileViewModel,
    vm_no_files_open::NoFilesOpenViewModel,
    vm_opening_files::OpeningFileViewModel,
    vm_frames_tool::FramesToolViewModel,
    vm_user_messages_tool::UserMessagesToolViewModel,
    vm_game_events_tool::GameEventsToolViewModel,
    vm_tasks_tool::TaskRunningViewModel,
};
use source_demo_tool::demo_file::{ DemoFile, frame::Command, packet::{FromProtobufMessagesWarnings, ParseMessageErr} };
use eframe::{egui::{ self, Key, Modifiers, Context, Layout }, emath::Align, epaint::Color32};
use std::{thread::{ self, JoinHandle }, sync::mpsc, time::SystemTime};

const SHIFT_JUMP_RANGE: usize = 10;
const INITIAL_UI_SCALE: f32 = 1.15;

pub struct MainViewModel {
    inner_view_model: Box<dyn ViewModel>,
    opening_file_join_handle: Option<JoinHandle<Result<DemoFile, String>>>,
    initializing_gui_join_handle: Option<JoinHandle<DemoFileViewModel>>,
    focused_vm: Focusable,
    ui_ppt: f32,
    task_join_handle: Option<JoinHandle<()>>,
    temporary_view_model: Option<Box<dyn ViewModel>>,
}

impl MainViewModel {
    pub fn new() -> Self {
        MainViewModel {
            inner_view_model: Box::new(NoFilesOpenViewModel{}),
            opening_file_join_handle: None,
            initializing_gui_join_handle: None,
            focused_vm: Focusable::None,
            ui_ppt: INITIAL_UI_SCALE,
            task_join_handle: None,
            temporary_view_model: None,
        }
    }

    fn handle_emit_netmsg_warnerrs(&mut self) -> bool {
        let df_vm_res = self.inner_view_model
            .as_any()
            .downcast_ref::<DemoFileViewModel>();

        match df_vm_res {
            Some(df_vm) => {
                let mut frames = df_vm.demo_file.frames.clone();
                frames.append(&mut df_vm.demo_file.sign_on_frames.clone());
                let (tx_percent_done, rx_percent_done) = mpsc::channel();

                self.task_join_handle = Some(thread::spawn(move || {
                    let mut last_update_time = SystemTime::now();
                    for i in 0..frames.len() {
                        let frame = frames[i].clone();
                        if let Command::Packet(pd) | Command::SignOn(pd) = frame.command {
                            for nmsg_ret in &pd.network_messages {
                                if let Some(warns) = &nmsg_ret.warnings {
                                    print_proto_warns(format!("PacketData/SignOnData_NetMessage[{}]", i).as_str(), warns);
                                } else if let Some(err) = &nmsg_ret.err {
                                    print_proto_err(format!("PacketData/SignOnData_NetMessage[{}]", i).as_str(), err);
                                }
                            }

                        } else if let Command::DataTables(dtd) = &frame.command {
                            for (_, warns) in &dtd.send_tables {
                                print_proto_warns(format!("DataTablesData_NetMessage[{}]", i).as_str(), warns)
                            }
                        }

                        if let Ok(etime) = last_update_time.elapsed() {
                            if etime.as_millis() >= 50 {
                                tx_percent_done.send(
                                    format!(
                                        "{}/{} frames",
                                        i + 1,
                                        frames.len()
                                    )
                                ).unwrap();
                                last_update_time = SystemTime::now();
                            }
                        }
                    }
                }));
                self.temporary_view_model = Some(Box::new(
                    TaskRunningViewModel::new(
                        "Dumping NetMessage warns/errs to console.",
                        rx_percent_done
                    )
                ));

            },
            None => eprintln!("Got Event::EmitNetMessageWarnErrs but no DemoFileViewModel present.")
        }

        true
    }

    fn handle_emit_usermsg_warnerrs(&mut self) -> bool {
        let df_vm_res = self.inner_view_model
            .as_any()
            .downcast_ref::<DemoFileViewModel>();

        match df_vm_res {
            Some(df_vm) => {
                let messages = df_vm.demo_file.get_user_messages();
                let (tx_percent_done, rx_percent_done) = mpsc::channel();

                self.task_join_handle = Some(thread::spawn(move || {
                    let mut last_update_time = SystemTime::now();
                    for i in 0..messages.len() {
                        let msg = &messages[i].message_return;

                        if let Some(warns) = &msg.warnings {
                            print_proto_warns(format!("UserMessage[{}]", i).as_str(), warns);
                        } else if let Some(err) = &msg.err {
                            print_proto_err(format!("UserMessage[{}]", i).as_str(), err);
                        }

                        if let Ok(etime) = last_update_time.elapsed() {
                            if etime.as_millis() >= 50 {
                                tx_percent_done.send(
                                    format!(
                                        "{}/{} messages",
                                        i + 1,
                                        messages.len()
                                    )
                                ).unwrap();
                                last_update_time = SystemTime::now();
                            }
                        }
                    }
                }));
                self.temporary_view_model = Some(Box::new(
                    TaskRunningViewModel::new(
                        "Dumping UserMessage warns/errs to console.",
                        rx_percent_done
                    )
                ));

            },
            None => eprintln!("Got Event::EmitUserMessageWarnErrs but no DemoFileViewModel present.")
        }

        true
    }

    fn handle_begin_open_file(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("Source Demo File", &["dem"])
            .pick_file();

        if let Some(path) = file {
            if path.exists() {
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                self.inner_view_model = Box::new(OpeningFileViewModel::new(name));

                self.opening_file_join_handle = Some(thread::spawn(move || {
                        DemoFile::open(&path)
                    })
                );
            } else {
                rfd::MessageDialog::new()
                    .set_level(rfd::MessageLevel::Error)
                    .set_description("file does not exist")
                    .set_title("Error")
                    .show();
                }
            }
        }

    fn handle_keyboard_events(&mut self, ctx: &Context, events: &mut Vec<Event>) {
        let b_shift = ctx.input(|i| i.modifiers.shift_only());
        let b_ctrl = ctx.input(|i| i.modifiers.command_only());
        // handle key up / down for lists
        let b_pressed_arrow_up = ctx.input(|i| i.key_pressed(Key::ArrowUp));
        let b_pressed_arrow_dn = ctx.input(|i| i.key_pressed(Key::ArrowDown));
        if b_pressed_arrow_dn || b_pressed_arrow_up {
            match &self.focused_vm {
                Focusable::FramesListViewModel => {
                    let df_vm_res = self.inner_view_model
                        .as_any_mut()
                        .downcast_mut::<DemoFileViewModel>();

                    if let Some(df_vm) = df_vm_res {
                        let frames_vm_res = df_vm.get_active_tool()
                            .as_any_mut()
                            .downcast_mut::<FramesToolViewModel>();

                        if let Some(frames_vm) = frames_vm_res {
                            if b_pressed_arrow_dn {
                                if b_ctrl {
                                    frames_vm.last_frame();
                                } else if b_shift {
                                    for _ in 0..SHIFT_JUMP_RANGE {
                                        frames_vm.next_frame();
                                    }
                                } else {
                                    frames_vm.next_frame();
                                }
                            }
                            if b_pressed_arrow_up {
                                if b_ctrl {
                                    frames_vm.first_frame();
                                } else if b_shift {
                                    for _ in 0..SHIFT_JUMP_RANGE {
                                        frames_vm.prev_frame();
                                    }
                                } else {
                                    frames_vm.prev_frame();
                                }
                            }
                        } else {
                            eprintln!("Focus was FramesListViewModel but no FramesToolViewModel present.");
                        }
                    } else {
                        eprintln!("Focus was FramesListViewModel but no DemoFileViewModel present.");
                    }
                },
                Focusable::ProtobufMessageListViewModel("packet_data_messages") => {
                    let df_vm_res = self.inner_view_model
                        .as_any_mut()
                        .downcast_mut::<DemoFileViewModel>();

                    if let Some(df_vm) = df_vm_res {
                        let frames_vm_res = df_vm.get_active_tool()
                            .as_any_mut()
                            .downcast_mut::<FramesToolViewModel>();

                        if let Some(frames_vm) = frames_vm_res {
                            if let Some(pd_vm) = &mut frames_vm.vm_packet_data {
                                if b_pressed_arrow_dn {
                                    if b_ctrl {
                                        pd_vm.vm_message_list.last_message();
                                    } else {
                                        pd_vm.vm_message_list.next_message();
                                    }
                                }
                                if b_pressed_arrow_up {
                                    if b_ctrl {
                                        pd_vm.vm_message_list.first_message();
                                    } else {
                                        pd_vm.vm_message_list.prev_message();
                                    }
                                }
                            } else {
                                eprintln!("Focus was packet_data_messages but no PacketDataViewModel present");
                            }
                        } else {
                            eprintln!("Focus was packet_data_messages but no FramesToolViewModel present");
                        }
                    } else {
                        eprintln!("Focus was packet_data_messages but no DemoFileViewModel present.");
                    }
                },
                Focusable::ProtobufMessageListViewModel("user_messages") => {
                    let df_vm_res = self.inner_view_model
                        .as_any_mut()
                        .downcast_mut::<DemoFileViewModel>();

                    if let Some(df_vm) = df_vm_res {
                        let um_vm_res = df_vm.get_active_tool()
                            .as_any_mut()
                            .downcast_mut::<UserMessagesToolViewModel>();

                        if let Some(um_vm) = um_vm_res {
                            if b_pressed_arrow_dn {
                                if b_ctrl {
                                    um_vm.vm_messages.last_message();
                                } else if b_shift {
                                    for _ in 0..SHIFT_JUMP_RANGE {
                                        um_vm.vm_messages.next_message();
                                    }
                                } else {
                                    um_vm.vm_messages.next_message();
                                }
                            }
                            if b_pressed_arrow_up {
                                if b_ctrl {
                                    um_vm.vm_messages.first_message();
                                } else if b_shift {
                                    for _ in 0..SHIFT_JUMP_RANGE {
                                        um_vm.vm_messages.prev_message();
                                    }
                                } else {
                                    um_vm.vm_messages.prev_message();
                                }
                            }
                        } else {
                            eprintln!("Focus was user_messages but no UserMessagesToolViewModel present.");
                        }
                    } else {
                        eprintln!("Focus was user_messages but no DemoFileViewModel present.");
                    }
                },
                Focusable::ProtobufMessageListViewModel(s) => {
                    eprintln!("Unknown ProtobufMessageListViewModel focusable id: {}", s);
                },
                Focusable::GameEventsList => {
                    let df_vm_res = self.inner_view_model
                    .as_any_mut()
                    .downcast_mut::<DemoFileViewModel>();

                    if let Some(df_vm) = df_vm_res {
                        let ge_vm_res = df_vm.get_active_tool()
                        .as_any_mut()
                        .downcast_mut::<GameEventsToolViewModel>();

                        if let Some(ge_vm) = ge_vm_res {
                            if b_pressed_arrow_dn {
                                if b_ctrl {
                                    ge_vm.last_message();
                                } else if b_shift {
                                    for _ in 0..SHIFT_JUMP_RANGE {
                                        ge_vm.next_message();
                                    }
                                } else {
                                    ge_vm.next_message();
                                }
                            }
                            if b_pressed_arrow_up {
                                if b_ctrl {
                                    ge_vm.first_message();
                                } else if b_shift {
                                    for _ in 0..SHIFT_JUMP_RANGE {
                                        ge_vm.prev_message();
                                    }
                                } else {
                                    ge_vm.prev_message();
                                }
                            }
                        } else {
                            eprintln!("Focus was GameEventsList but no GameEventsToolViewModel present.");
                        }
                    } else {
                        eprintln!("Focus was GameEventsList but no DemoFileViewModel present.");
                    }
                },
                Focusable::SendTables => {
                    let df_vm_res = self.inner_view_model
                        .as_any_mut()
                        .downcast_mut::<DemoFileViewModel>();

                    if let Some(df_vm) = df_vm_res {
                        let frames_vm_res = df_vm.get_active_tool()
                            .as_any_mut()
                            .downcast_mut::<FramesToolViewModel>();

                        if let Some(frames_vm) = frames_vm_res {
                            if let Some(dt_vm) = &mut frames_vm.vm_data_tables {
                                if dt_vm.active_mode == "Send Tables" {
                                    if b_pressed_arrow_dn {
                                        if b_ctrl {
                                            dt_vm.send_table_last();
                                        } else if b_shift {
                                            for _ in 0..SHIFT_JUMP_RANGE {
                                                dt_vm.send_table_next();
                                            }
                                        } else {
                                            dt_vm.send_table_next();
                                        }
                                    }
                                    if b_pressed_arrow_up {
                                        if b_ctrl {
                                            dt_vm.send_table_first();
                                        } else if b_shift {
                                            for _ in 0..SHIFT_JUMP_RANGE {
                                                dt_vm.send_table_prev();
                                            }
                                        } else {
                                            dt_vm.send_table_prev();
                                        }
                                    }
                                }
                            } else {
                                eprintln!("Focus was SendTables but no DataTablesViewModel present");
                            }
                        } else {
                            eprintln!("Focus was SendTables but no FramesToolViewModel present");
                        }
                    } else {
                        eprintln!("Focus was SendTables but no DemoFileViewModel present.");
                    }
                },
                Focusable::None => {}, // do nothing
            }
        }

        // handle key left / right for switching tools
        let b_pressed_arrow_left = ctx.input(|i| i.key_pressed(Key::ArrowLeft));
        let b_pressed_arrow_right = ctx.input(|i| i.key_pressed(Key::ArrowRight));
        if b_pressed_arrow_left || b_pressed_arrow_right {
            let df_vm_res = self.inner_view_model
                .as_any_mut()
                .downcast_mut::<DemoFileViewModel>();

            if let Some(df_vm) = df_vm_res {
                if b_pressed_arrow_left {
                    if b_ctrl {
                        df_vm.first_tool();
                    } else {
                        df_vm.prev_tool();
                    }
                }
                if b_pressed_arrow_right {
                    if b_ctrl {
                        df_vm.last_tool();
                    } else {
                        df_vm.next_tool();
                    }
                }
            }
        }

        // Ctrl+O: Open file anywhere in program
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::O)) {
            events.push(Event::BeginOpenFile);
        }
    }

    fn handle_opening_file(&mut self, events: &mut Vec<Event>) {
        if let Some(jh) = self.opening_file_join_handle.take() {
            if jh.is_finished() {
                match jh.join().unwrap() {
                    Ok(df) => {
                        events.push(Event::NewFile(df.path.clone()));
                        self.initializing_gui_join_handle = Some(thread::spawn(move || {
                            DemoFileViewModel::new(df)
                        }));
                    },
                    Err(e) => {
                        rfd::MessageDialog::default()
                            .set_description(format!("Failed to open file: {}", e).as_str())
                            .set_title("Error")
                            .set_level(rfd::MessageLevel::Error)
                            .show();
                        self.inner_view_model = Box::new(NoFilesOpenViewModel{});
                    }
                }
            } else {
                self.opening_file_join_handle = Some(jh);
            }
        }
    }

    fn handle_initializing_gui(&mut self) {
        if let Some(jh) = self.initializing_gui_join_handle.take() {
            if jh.is_finished() {
                if let Ok(df_vm) = jh.join() {
                    self.inner_view_model = Box::new(df_vm);
                }
            } else {
                self.initializing_gui_join_handle = Some(jh);
            }
        }
    }

    fn set_styles(ui: &mut egui::Ui) {
        ui.style_mut()
            .visuals
            .extreme_bg_color = Color32::from_gray(32);
    }
}

impl ViewModel for MainViewModel {
    fn draw(&mut self, ui: &mut egui::Ui, events: &mut Vec<Event>) {
        self.handle_opening_file(events);
        self.handle_initializing_gui();
        self.handle_keyboard_events(ui.ctx(), events);
        Self::set_styles(ui);

        if let Some(task_jh) = self.task_join_handle.take() {
            if task_jh.is_finished() {
                task_jh.join().unwrap();
                self.temporary_view_model = None;
                eprintln!("Task Complete");
            } else {
                self.task_join_handle = Some(task_jh);
            }
        }

        let mut ui_scale = self.ui_ppt;
        ui.vertical(|ui| {
            let avail_width = ui.available_width();

            egui::Grid::new("main_ui_header_grid")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(avail_width / 2.0);
                    // nothing for now
                });
                ui.with_layout(
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.set_width(avail_width / 2.0);
                        ui.add_space(20.0);
                        if ui.add(egui::Slider::new(&mut ui_scale, 0.75..=2.0))
                        .drag_released() {
                            self.ui_ppt = ui_scale;
                        }
                        ui.label("UI Scale (ppt)");
                });
            });
            ui.separator();

            if let Some(temp_vm) = self.temporary_view_model.as_mut() {
                temp_vm.draw(ui, events);
            } else {
                self.inner_view_model.draw(ui, events);
            }
        });

        ui.ctx().set_pixels_per_point(self.ui_ppt);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::BeginOpenFile => {
                self.handle_begin_open_file();
                return true
            },
            Event::SetFocus(focusable) => {
                self.focused_vm = focusable.clone();
                // let inner grab this event as well
                return self.inner_view_model.handle_event(event)
            },
            Event::EmitNetMsgWarnErrs => return self.handle_emit_netmsg_warnerrs(),
            Event::EmitUserMsgWarnErrs => return self.handle_emit_usermsg_warnerrs(),
            _ => {}
        }

        self.inner_view_model.handle_event(event)
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

fn print_proto_warns_inner(name: &str, depth: usize, warns: &FromProtobufMessagesWarnings) {
    if warns.has_warnings() {
        eprintln!(
            "== {} ProtobufMessageWarnings depth: {} ==",
            name, depth
        );
        if !warns.missing_fields.is_empty() {
            eprintln!(
                "# Missing Fields\n{:#?}",
                warns.missing_fields
            );
        }
        if !warns.unknown_fields.is_empty() {
            eprintln!(
                "# Unknown Fields\n{:#?}",
                warns.unknown_fields
            );
        }
        if !warns.repeated_fields.is_empty() {
            eprintln!(
                "# Repeated Fields\n{:?}",
                warns.repeated_fields
            );
        }
        for sub_warn in &warns.sub_warnings {
            print_proto_warns_inner(sub_warn.0, depth + 1, &sub_warn.1);
        }
    }
}

pub fn print_proto_warns(name: &str, warns: &FromProtobufMessagesWarnings) {
    print_proto_warns_inner(name, 0, warns);
}

pub fn print_proto_err(name: &str, err: &ParseMessageErr) {
    eprintln!(
        "== {} ProtobufMessageParseErr ==\n{:?}",
        name,
        err
    );
}