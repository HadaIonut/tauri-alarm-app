use chrono::{DateTime, Local};
use nanoid::nanoid;
use crate::schedule_thread::alarms_changed_payload::{AlarmRemovedOrAddedPayload, SerializableScheduleMessage};

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

    pub fn serialize(&self) -> SerializableScheduleMessage {
        SerializableScheduleMessage::new(self.clone())
    }

    pub fn to_alarm_payload(&self) -> AlarmRemovedOrAddedPayload {
        AlarmRemovedOrAddedPayload::new(self.clone())
    }
}
