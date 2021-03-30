use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    Create(Event),
    Remove(EventFilter),
    Update(EventFilter, Event),
    Query(EventFilter),
    None
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub trigger: EventTrigger,
    pub name: String,
    pub data: HashMap<String, String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventFilter {
    pub trigger: EventTrigger,
    pub tags: Vec<String>,
}
