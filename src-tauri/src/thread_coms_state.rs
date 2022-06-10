use std::sync::mpsc::{Sender};
use crate::ScheduleMessage;

pub struct ThreadComsState {
    pub sender:Sender<ScheduleMessage>
}

unsafe impl Sync for ThreadComsState {}