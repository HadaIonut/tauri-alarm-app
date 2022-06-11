use chrono::{DateTime, Local, TimeZone};
use tauri::State;
use crate::ScheduleMessage;
use crate::thread_coms_state::ThreadComsState;

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
pub fn create_new_alarm(state: State<ThreadComsState>, message: String, timestamp: String) {
    let vec: Vec<&str> = timestamp.split(" ").collect();
    state.sender.send(ScheduleMessage::new(
        message,
        timestamp_from_date_and_time(vec[0].parse().unwrap(), vec[1].parse().unwrap())
    )).unwrap();
}

