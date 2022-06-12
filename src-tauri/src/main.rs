#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use crate::schedule_thread::schedule_message::ScheduleMessage;

mod commands;
mod schedule_thread;

fn main() {
    tauri::Builder::default()
        .manage(schedule_thread::RunningAlarms::default())
        .invoke_handler(tauri::generate_handler![
            commands::create_new_alarm,
            schedule_thread::start_schedule_thread])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
