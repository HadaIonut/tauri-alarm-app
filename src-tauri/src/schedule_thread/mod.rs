use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::{thread, time};
use chrono::Local;
use tauri::{State, Window, Wry};
use crate::ScheduleMessage;
use std::process::Command;

pub mod schedule_message;
pub mod alarms_changed_payload;

#[derive(Default)]
pub struct RunningAlarms {
    pub time_stamp_map: Arc<Mutex<HashMap<String, ScheduleMessage>>>,
}

fn trigger_notification(notification_message: String) {
    let command = "zenity --info --title \"Tauri Alarm\" --text \"".to_string()
        + &*notification_message + &*"\" --timeout=2".to_string();
    let mut notification = Command::new("sh");
    notification.arg("-c").arg(command).output().expect("failed to execute process");
}

fn play_sound() {
    use soloud::*;
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../../alarm/alarm1.wav")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        sleep(time::Duration::from_millis(100));
    }
}

fn trigger_alarm(notification_message: String) {
    thread::spawn(move || { trigger_notification(notification_message)});
    thread::spawn(move || { play_sound()});

}

#[tauri::command(async)]
pub fn start_schedule_thread(to_start_alarms_state: State<RunningAlarms>, window: Window<Wry>) {
    let state_clone = Arc::clone(&to_start_alarms_state.time_stamp_map);
    thread::spawn(move || {
        loop {
            let mut data = state_clone.lock().unwrap();

            if data.len() > 0 {
                data.retain(|_key, val| -> bool {
                    if val.execute_time_stamp.timestamp() <= Local::now().timestamp() {
                        trigger_alarm(val.message.clone());
                        let payload = val.to_alarm_payload();
                        window.emit("alarm-removed",  payload).unwrap();
                    }

                    val.execute_time_stamp.timestamp() > Local::now().timestamp()
                })
            }

            drop(data);
            sleep(time::Duration::from_millis(1000));
        }
    });
}
