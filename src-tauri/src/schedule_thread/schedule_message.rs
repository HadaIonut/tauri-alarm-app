use nanoid::nanoid;
use crate::schedule_thread::alarms_changed_payload::{AlarmRemovedOrAddedPayload, SerializableScheduleMessage};
use chrono::{DateTime, Local, TimeZone};

pub fn timestamp_from_date_and_time(date: String, time: String) -> DateTime<Local> {
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


#[derive(Clone)]
pub struct ScheduleMessage {
    pub id: String,
    pub message: String,
    pub execute_time_stamp: DateTime<Local>,
}

impl ScheduleMessage {
    pub fn new(message: String, execute_time_stamp: DateTime<Local>) -> ScheduleMessage {
        ScheduleMessage { id: nanoid!(), message, execute_time_stamp }
    }

    pub fn new_from_strings(message: String, timestamp: String) -> ScheduleMessage {
        let vec: Vec<&str> = timestamp.split(" ").collect();

        ScheduleMessage::new(
            message,
            timestamp_from_date_and_time(vec[0].parse().unwrap(),
                                         vec[1].parse().unwrap())
        )
    }

    pub fn new_from_strings_with_id(id: String, message: String, timestamp: String) -> ScheduleMessage {
        let vec: Vec<&str> = timestamp.split(" ").collect();

        ScheduleMessage {
            id,
            message,
            execute_time_stamp: timestamp_from_date_and_time(vec[0].parse().unwrap(),
            vec[1].parse().unwrap())
        }
    }

    pub fn serialize(&self) -> SerializableScheduleMessage {
        SerializableScheduleMessage::new(self.clone())
    }

    pub fn to_alarm_payload(&self) -> AlarmRemovedOrAddedPayload {
        AlarmRemovedOrAddedPayload::new(self.clone())
    }
}
