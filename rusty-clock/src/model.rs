use chrono::{DateTime, Local};

pub struct RustyClock {
    pub start_time: Option<DateTime<Local>>,
    pub log: Vec<(DateTime<Local>, DateTime<Local>, String, f64)>,
    pub current_description: String,
}

impl Default for RustyClock {
    fn default() -> Self {
        Self {
            start_time: None,
            log: vec![],
            current_description: String::new(),
        }
    }
}
