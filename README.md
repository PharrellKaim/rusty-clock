## ⏰ Rusty Clock - Your Simple, Local Time Tracker
Rusty Clock is a minimalist desktop time-tracking application built with Rust and the egui framework (via eframe). The tool is aimed at developers and freelancers who need an uncomplicated, non-cloud-based solution.

All time entries are stored locally in a timelog.json file.

## ✨ Features
Start/Stop Functionality: Easily start and end work sessions.

Real-time Display: Tracking of the current session duration and the total working time for the current day.

Description: Capture a short description for each log entry.

Local Storage: All data is automatically saved in a JSON file (timelog.json).

## 🛠️ Installation and Setup
This project requires Rust (with Cargo) to be installed on your system.

1. Dependencies

The necessary crates are defined in Cargo.toml. Make sure you have activated the correct features for chrono and serde:

[dependencies]
eframe = "0.27"
egui = "0.27"
chrono = { version = "0.4", features = ["local", "serde"] } # 'serde' feature is necessary for JSON serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

### 2. Build and Run
Clone the repository and start the application using Cargo:
```
# Compile and start the app 
cargo run
```


The application will then start in its own window.

## 🚀 Usage
Enter Description: Enter a brief description of the current work in the text field (e.g., "Bug fixing in module X" or "Concept for feature Y").

Start: Click the Start button to begin time tracking.

Stop: Click the Stop button to end the current session and save it as a log entry.

Data: All completed sessions are stored as a time tuple (Start Time, End Time, Description) in the timelog.json file in the project's root directory.