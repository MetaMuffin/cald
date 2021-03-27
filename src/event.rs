use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EventTrigger {
    Always,
    Never,
    Is(TimeComponent),
    Divisible(TimeComponent),
    OneOf(Vec<EventTrigger>),
    AllOf(Vec<EventTrigger>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeComponent {
    Years(u32),
    MonthYear(u8),
    DayMonth(u8),
    DayWeek(u8),
    HourDay(u8),
    MinuteHour(u8),
    SecondMinute(u8),
}

pub struct Tag {
    action: String,
}

pub struct Event {
    trigger: EventTrigger,
    name: String,
    data: HashMap<String, String>,
    tags: Vec<String>,
}

pub struct EventFilter {
    trigger: EventTrigger,
    tags: Vec<String>,
}
