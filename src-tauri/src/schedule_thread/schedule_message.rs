use chrono::{DateTime, Local};
use nanoid::nanoid;

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
}
