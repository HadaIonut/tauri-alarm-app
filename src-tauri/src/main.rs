#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::schedule_thread::schedule_message::ScheduleMessage;

mod commands;
mod schedule_thread;
mod thread_coms_state;

fn main() {
    let (sender, receiver): (Sender<ScheduleMessage>,
                             Receiver<ScheduleMessage>) = mpsc::channel();

    thread::spawn(move || schedule_thread::schedule_alert(receiver));

    tauri::Builder::default()
        .manage(thread_coms_state::ThreadComsState {
            sender
        })
        .invoke_handler(tauri::generate_handler![commands::create_new_alarm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
