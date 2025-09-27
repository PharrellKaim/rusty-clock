mod app;
mod ui;
mod model;

use model::RustyClock;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Rusty Clock - Time Tracker",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([800.0, 600.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(RustyClock::default()))),
    )
}
