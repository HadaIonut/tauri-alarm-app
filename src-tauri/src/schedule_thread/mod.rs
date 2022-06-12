use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::{thread, time};
use chrono::Local;
use tauri::{State, Window, Wry};
use crate::ScheduleMessage;

pub mod schedule_message;
pub mod alarms_changed_payload;

#[derive(Default)]
pub struct RunningAlarms {
    pub time_stamp_map: Arc<Mutex<HashMap<String, ScheduleMessage>>>,
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
                        println!("Executed: {}", val.message);
                        let payload = alarms_changed_payload::AlarmRemovedOrAddedPayload::new(val.clone());
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
