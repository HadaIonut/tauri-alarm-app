use crate::{ScheduleMessage};

#[derive(Clone, serde::Serialize)]
pub struct SerializableScheduleMessage {
    pub id: String,
    pub message: String,
    pub execute_time_stamp: String,
}

impl SerializableScheduleMessage {
    pub fn new(message: ScheduleMessage) -> SerializableScheduleMessage {
        SerializableScheduleMessage {
            id: message.id,
            message: message.message,
            execute_time_stamp: message.execute_time_stamp.format("%Y-%m-%d %H:%M").to_string()
        }
    }

    pub fn to_string(&self) -> String {
        "{".to_string() + &*format!("id:{}, message:{}, execute_time_stamp:{}",
                                    self.id,
                                    self.message,
                                    self.execute_time_stamp) + &*"}".to_string()
    }
}

#[derive(Clone, serde::Serialize)]
pub struct AlarmRemovedOrAddedPayload {
    alarm: SerializableScheduleMessage
}

impl AlarmRemovedOrAddedPayload {
    pub fn new(message: ScheduleMessage) -> AlarmRemovedOrAddedPayload {
        AlarmRemovedOrAddedPayload {
            alarm: SerializableScheduleMessage::new(message)
        }
    }
}