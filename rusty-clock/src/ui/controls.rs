use eframe::egui;
use crate::model::RustyClock;

pub fn show(ctx: &egui::Context, app: &mut RustyClock) {
    egui::TopBottomPanel::top("controlsPanel").show(ctx, |ui| {
        ui.set_max_height(60.0);
        ui.spacing_mut().item_spacing.x = 10.0;

        ui.horizontal(|ui| {
            if app.start_time.is_none() {
                if ui.button("Start").clicked() {
                    app.start_time = Some(chrono::Local::now());
                }
            } else {
                if ui.button("Stop").clicked() {
                    if let Some(start) = app.start_time.take() {
                        let end = chrono::Local::now();
                        app.log.push((start, end));

                        let json = serde_json::to_string(&app.log).unwrap();
                        std::fs::write("./timelog.json", json).unwrap();
                    }
                }
            }

            ui.separator();

            if let Some(start) = app.start_time {
                let now = chrono::Local::now();
                let elapsed = now - start;

                let seconds = elapsed.num_seconds();
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                let sec = seconds % 60;

                ui.horizontal(|ui| {
                    ui.label(format!("Current Session: {:02}:{:02}:{:02}", hours, minutes, sec));

                    let today = chrono::Local::now().date_naive();
                    let mut today_duration = chrono::Duration::zero();

                    for (s, e) in &app.log {
                        if s.date_naive() == today {
                            today_duration = today_duration + (*e - *s);
                        }
                    }

                    if start.date_naive() == today {
                        today_duration = today_duration + elapsed;
                    }

                    let secs_total = today_duration.num_seconds();
                    let h_total = secs_total / 3600;
                    let m_total = (secs_total % 3600) / 60;
                    let s_total = secs_total % 60;

                    ui.label(format!("📊 Total today: {:02}:{:02}:{:02}", h_total, m_total, s_total));
                });

                ctx.request_repaint();
            }
        });
    });
}
