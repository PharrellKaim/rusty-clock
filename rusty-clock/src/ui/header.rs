use eframe::egui;
use crate::model::RustyClock;

pub fn show(ctx: &egui::Context, app: &mut RustyClock) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("🕒 Rusty Clock – Time Tracker");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("💾 Save").clicked() {
                    let today = chrono::Local::now().date_naive();
                    let todays_sessions: Vec<_> = app
                        .log
                        .iter()
                        .filter(|(s, _, _, _)| s.date_naive() == today)
                        .collect();

                    let json = serde_json::to_string_pretty(&todays_sessions).unwrap();
                    let filename = format!("./output/timelog_{}.json", today.format("%Y-%m-%d"));
                    if let Err(e) = std::fs::write(filename, json) {
                        eprintln!("Error at saving: {e}");
                    } else {
                        println!("Saved!");
                    }
                }
            });
        });
    });
}
