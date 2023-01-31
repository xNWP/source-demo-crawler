mod gui;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1280.0, 720.0].into()),
        initial_window_pos: Some([20.0, 20.0].into()),
        ..Default::default()
    };
    eframe::run_native("Source Demo Crawler", native_options, Box::new(|cc| Box::new(gui::NewCrawlerApp::new(cc))));
}