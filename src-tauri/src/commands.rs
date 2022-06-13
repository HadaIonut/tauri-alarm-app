use chrono::{DateTime, Local, TimeZone};
use tauri::{State, Window, Wry};
use crate::{schedule_thread, ScheduleMessage};
use crate::schedule_thread::alarms_changed_payload::SerializableScheduleMessage;

#[tauri::command]
pub fn create_new_alarm(to_start_alarms_state: State<schedule_thread::RunningAlarms>, window: Window<Wry>, message: String, timestamp: String) {
    let msg = ScheduleMessage::new_from_strings(message, timestamp);
    let alarms_payload = msg.to_alarm_payload();
    to_start_alarms_state.time_stamp_map.lock().unwrap().insert(msg.id.clone(), msg);
    window.emit("alarm-added", alarms_payload).unwrap();
}

#[tauri::command]
pub fn init_file_save(to_start_alarms_state: State<schedule_thread::RunningAlarms>, alarms:Vec<SerializableScheduleMessage>) {
    let mut locked_state = to_start_alarms_state.time_stamp_map.lock().unwrap();
    for alarm in alarms {
        locked_state.insert(alarm.id.clone(), alarm.to_schedule_message());
    }
}
