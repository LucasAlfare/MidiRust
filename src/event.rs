#![allow(unused)]

#[derive(Debug)]
pub enum EventDataType {
    TextData(String),
    VecData(Vec<u32>)
}

#[derive(Debug)]
pub enum EventCategory {
    Meta,
    Control,
    SystemExclusive
}

#[derive(Debug)]
pub struct Event {
    pub category: EventCategory,
    pub delta_time: u32,
    pub data: Vec<EventDataType>
}

