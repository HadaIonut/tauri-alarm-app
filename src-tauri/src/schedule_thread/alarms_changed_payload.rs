use crate::{ScheduleMessage};

#[derive(Clone, serde::Serialize)]
struct SerializableScheduleMessage {
    pub id: String,
    pub message: String,
    pub execute_time_stamp: String,
}

#[derive(Clone, serde::Serialize)]
pub struct AlarmRemovedOrAddedPayload {
    alarm: SerializableScheduleMessage
}

impl AlarmRemovedOrAddedPayload {
    pub fn new(message: ScheduleMessage) -> AlarmRemovedOrAddedPayload {
        AlarmRemovedOrAddedPayload {
            alarm: SerializableScheduleMessage {
                id: message.id,
                message: message.message,
                execute_time_stamp: message.execute_time_stamp.format("%Y-%m-%d %H:%M").to_string()
            }
        }
    }
}