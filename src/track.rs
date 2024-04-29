#![allow(unused)]

use crate::{event::{Event, EventCategory}, my_reader::MyReader};

pub struct Track {
    pub signature: String,
    pub length: i32,
    pub events: Vec<Event>
}

impl Track {
    pub fn define(reader: &mut MyReader) {
        let _signature = reader.read_string(4);
        let _length = reader.read_4_bytes();
        
        let mut previous_status = 0;

        loop {
            let delta_time = reader.read_variable_length_value();
            let mut current_status = reader.read_1_byte();

            if current_status << 7 == 0 {
                current_status = previous_status;
                reader.offset -= 1;
            }

            match current_status {
                0xFF => {
                    let next_type = reader.read_1_byte();
                    let next_event_category = EventCategory::Meta;

                }

                _ => {}
            }
        }
    }
}
