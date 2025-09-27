use eframe::egui;
use crate::ui;
use crate::model::RustyClock;

impl eframe::App for RustyClock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::header::show(ctx, self);
        ui::controls::show(ctx, self);
        ui::sessions::show(ctx, self);
    }
}
