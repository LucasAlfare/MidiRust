#![allow(unused)]

use crate::{event::Event, my_reader::MyReader};

pub struct Track {
    pub signature: String,
    pub length: i32,
    pub events: Vec<Event>
}

impl Track {
    pub fn new(signature: String, length: i32) -> Self {
        Track {
            signature,
            length,
            events: Vec::new()
        }
    }

    pub fn define(self, reader: &mut MyReader) {
        println!("{:?}", self.events);
    }
}
