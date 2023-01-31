use super::{ ViewModel, w_copyable_field::CopyableFieldWidget };
use source_demo_tool::demo_file::header::DemoHeader;

pub struct HeaderToolViewModel {
    pub demo_header: DemoHeader,
}

impl HeaderToolViewModel {
    pub fn new(demo_header: DemoHeader) -> Self {
        Self { demo_header }
    }
}

impl ViewModel for HeaderToolViewModel {
    fn draw(&mut self, ui: &mut eframe::egui::Ui, _events: &mut Vec<super::Event>) {
        ui.add(CopyableFieldWidget {
            label: "Client Name".into(),
            value: Box::new(self.demo_header.client_name.clone()),
        });
        ui.add(CopyableFieldWidget {
            label: "Server Name".into(),
            value: Box::new(self.demo_header.server_name.clone()),
        });
        ui.add(CopyableFieldWidget {
            label: "Map Name".into(),
            value: Box::new(self.demo_header.map_name.clone()),
        });
        ui.add(CopyableFieldWidget {
            label: "Game Directory".into(),
            value: Box::new(self.demo_header.game_directory.clone()),
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.add(CopyableFieldWidget {
                label: "Playback Time".into(),
                value: Box::new(self.demo_header.playback_time),
            });
            ui.add(CopyableFieldWidget {
                label: "Ticks".into(),
                value: Box::new(self.demo_header.ticks),
            });
            ui.add(CopyableFieldWidget {
                label: "Frames".into(),
                value: Box::new(self.demo_header.frames),
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.add(CopyableFieldWidget {
                label: "Demo Protocol".into(),
                value: Box::new(self.demo_header.demo_protocol),
            });
            ui.add(CopyableFieldWidget {
                label: "Network Protocol".into(),
                value: Box::new(self.demo_header.network_protocol),
            });
        });
        ui.add(CopyableFieldWidget {
            label: "Sign On Length".into(),
            value: Box::new(self.demo_header.sign_on_length),
        });
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}