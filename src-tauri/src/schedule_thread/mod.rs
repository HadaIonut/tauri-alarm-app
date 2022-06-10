use std::collections::HashMap;
use std::thread::sleep;
use std::time;
use chrono::Local;
use std::sync::mpsc::{Receiver};
pub mod schedule_message;

pub fn schedule_alert(rx: Receiver<schedule_message::ScheduleMessage>) {
    let mut time_stamp_map = HashMap::new();
    loop {
        let received_wrapped = rx.try_recv();
        let received;
        if received_wrapped.is_ok() {
            received = received_wrapped.unwrap();
            println!("Received: {}", received.message);
            time_stamp_map.insert(received.clone().id, received);
            //TODO: fire time_stamp_map_changed event
        }

        if time_stamp_map.len() > 0 {
            time_stamp_map.retain(|_key, val| -> bool {
                if val.execute_time_stamp.timestamp() <= Local::now().timestamp() {
                    println!("Executed: {}", val.message);
                    //TODO: fire time_stamp_map_changed event
                }

                val.execute_time_stamp.timestamp() > Local::now().timestamp()
            });
        }

        sleep(time::Duration::from_millis(100));
    }
}