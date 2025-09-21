use eframe::{egui, NativeOptions};
use chrono::{Local, DateTime};
use std::{fs, time::Duration};

struct RustyClock {
    start_time: Option<DateTime<Local>>,
    log: Vec<(DateTime<Local>, DateTime<Local>)>
}

impl Default for RustyClock {
    fn default() -> Self {
        Self { 
            start_time: None,
            log: vec![],
        }
    }    
}

impl eframe::App for RustyClock {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            
            ui.heading("🕒 Rusty Clock – Time Tracker");

            if self.start_time.is_none() {
                if ui.button("Start").clicked() {
                    self.start_time = Some(Local::now());
                }
            } else {
                if ui.button("Stop").clicked() {
                    if let Some(start) = self.start_time.take(){
                        let end = Local::now();
                        self.log.push((start, end));

                        let json = serde_json::to_string(&self.log).unwrap();
                        std::fs::write("timelog.json", json).unwrap();
                    }
                }
            }

            ui.separator();

            use chrono::{Local, Duration, NaiveDate};

            if let Some(start) = self.start_time{
                let now = Local::now();
                let elapsed = now - start;

                let seconds = elapsed.num_seconds();
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                let sec = seconds % 60;     

                ui.horizontal(|ui| { 
                ui.label(format!("Current Session: {:02}:{:02}:{02}", hours, minutes, sec)); 

                // Todays Session
                let today = Local::now().date_naive() ;
                let mut today_duration = Duration::zero();

                // sum up all finished sessions
                for (s, e) in &self.log {
                    if s.date_naive() == today {
                        today_duration = today_duration + (*e - *s);
                    }
                }  

                // add current running session
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

            ui.separator();         
            ui.label("Previous Session: ");
            for (start, end) in &self.log {
                let duration = *end - *start;
                let seconds = duration.num_seconds();
                let hours = seconds /3600;
                let minutes = (seconds % 3600) / 60;
                let sec = seconds % 60;

                ui.label(format!("
                {} - {} (⏱️ {:02}:{:02}:{:02})",
                start.format("%Y-%m-%d %H:%M:%S"),
                end.format("%Y-%m-%d %H:%M:%S"),
                hours, minutes, sec
                ));
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Rusty Clock - Time Tracker",
        eframe::NativeOptions{
            viewport: egui::ViewportBuilder::default().with_inner_size([800.0,600.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(RustyClock::default())))
    )
}