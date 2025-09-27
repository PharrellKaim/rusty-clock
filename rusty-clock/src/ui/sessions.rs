use eframe::egui;
use crate::model::RustyClock;

pub fn show(ctx: &egui::Context, app: &mut RustyClock) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.separator();
        ui.label("Previous Sessions:");

        for (start, end, description) in &app.log {
            let duration = *end - *start;
            let seconds = duration.num_seconds();
            let hours = seconds / 3600;
            let minutes = (seconds % 3600) / 60;
            let sec = seconds % 60;

            ui.label(format!(
                "{}: {} - {} (⏱️ {:02}:{:02}:{:02})",
                description,
                start.format("%Y-%m-%d %H:%M:%S"),
                end.format("%Y-%m-%d %H:%M:%S"),
                hours, minutes, sec
            ));
        }
    });
}
