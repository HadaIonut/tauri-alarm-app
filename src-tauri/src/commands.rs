use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use chrono::{DateTime, Local, TimeZone};
use tauri::{Manager, State, Window, Wry};
use crate::{schedule_thread, ScheduleMessage};

fn timestamp_from_date_and_time(date: String, time: String) -> DateTime<Local> {
    let mut date_time_array: [u32; 5] = [0;5];

    let mut index = 0;
    for s in date.split("-") {
        date_time_array[index] = s.to_string().parse::<u32>().unwrap();
        index += 1;
    }

    for s in time.split(":") {
        date_time_array[index] = s.to_string().parse::<u32>().unwrap();
        index += 1;
    }

    Local.ymd(date_time_array[0] as i32, date_time_array[1], date_time_array[2])
        .and_hms(date_time_array[3], date_time_array[4], 0)
}

#[tauri::command]
pub fn create_new_alarm(to_start_alarms_state: State<schedule_thread::RunningAlarms>, window: Window<Wry>, message: String, timestamp: String) {
    let vec: Vec<&str> = timestamp.split(" ").collect();
    let msg = ScheduleMessage::new(
            message,
            timestamp_from_date_and_time(vec[0].parse().unwrap(), vec[1].parse().unwrap())
        );
    let alarms_payload = msg.to_alarm_payload();
    to_start_alarms_state.time_stamp_map.lock().unwrap().insert(msg.id.clone(), msg);
    window.emit("alarm-added", alarms_payload).unwrap();

}

#[tauri::command]
pub fn init_file_save(to_start_alarms_state: State<schedule_thread::RunningAlarms>, window: Window<Wry>) {
    let state_clone = Arc::clone(&to_start_alarms_state.time_stamp_map);

    window.listen_global("alarm-write", move|_event| {
        let mut file = File::create("foo.txt").expect("error");

        let mut schedule_string: String = "[".to_string();
        for value in state_clone.lock().unwrap().values() {
            let serialized = value.serialize();
            schedule_string = schedule_string + &*serialized.to_string();
            schedule_string = schedule_string + ",";
        }
        schedule_string.pop();
        schedule_string = schedule_string + "]";

        if schedule_string == "]" { schedule_string = "".to_string(); }

        file.write_all(schedule_string.as_ref()).expect("TODO: panic message");
    });
}
