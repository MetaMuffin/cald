use std::fmt::Display;

use crate::event::{Event, EventFilter, EventTrigger, Operation, TimeComponent};

pub fn ident(s: String) -> String {
    return format!("  {}", s).replace("\n", "\n  ");
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operation::Create(ev) => format!("create event: \n{}", ident(format!("{:#}", ev))),
            Operation::Remove(evf) => format!(
                "remove event matched by a filter: \n{}",
                ident(format!("{:#}", evf))
            ),
            Operation::Update(evf, ev) => format!(
                "Update (aka replace) events matched by a filter with: \n{}\n{}",
                ident(format!("{:#}", evf)),
                ident(format!("{:#}", ev))
            ),
            Operation::Query(evf) => format!(
                "show every event matched by a filter: \n{}",
                ident(format!("{:#}", evf))
            ),
            Operation::None => format!("nothing aka no-op"),
        };
        f.write_fmt(format_args!("{}", s))
    }
}

impl Display for EventFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!() // todo implement std::fmt::Display for EventFilter
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:\n  trigger:\n  {}\n  tags: {}",
            self.name,
            ident(format!("{:#}", self.trigger)),
            self.tags.join(", ")
        ))
    }
}

impl Display for EventTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EventTrigger::Always => format!("always"),
            EventTrigger::Never => format!("never"),
            EventTrigger::Is(v) => format!("{} is {}", v.get_time_component_name(), v.get_value()),
            EventTrigger::Divisible(v) => format!(
                "{} is divisible by {}",
                v.get_time_component_name(),
                v.get_value()
            ),
            EventTrigger::OneOf(cs) => format!(
                "when one of the condition matches:\n{}",
                cs.iter()
                    .map(|e| ident(format!("  {:#}", e)))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            EventTrigger::AllOf(cs) => format!(
                "when all condition match:\n{}",
                cs.iter()
                    .map(|e| ident(format!("  {:#}", e)))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        };
        f.write_fmt(format_args!("{}", s))
    }
}

impl Display for TimeComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: {}",
            self.get_time_component_name(),
            self.get_value()
        ))
    }
}

impl TimeComponent {
    pub fn get_time_component_name(&self) -> String {
        match self {
            TimeComponent::Years(_) => String::from("year"),
            TimeComponent::MonthYear(_) => String::from("month of the year"),
            TimeComponent::DayMonth(_) => String::from("day of the month"),
            TimeComponent::DayWeek(_) => String::from("day of the week"),
            TimeComponent::HourDay(_) => String::from("hour"),
            TimeComponent::MinuteHour(_) => String::from("minute"),
            TimeComponent::SecondMinute(_) => String::from("second"),
            TimeComponent::WeekYear(_) => String::from("week of the year"),
        }
    }
    pub fn get_value(&self) -> u32 {
        match self {
            TimeComponent::Years(v) => *v,
            TimeComponent::MonthYear(v) => *v as u32,
            TimeComponent::DayMonth(v) => *v as u32,
            TimeComponent::DayWeek(v) => *v as u32,
            TimeComponent::HourDay(v) => *v as u32,
            TimeComponent::MinuteHour(v) => *v as u32,
            TimeComponent::SecondMinute(v) => *v as u32,
            TimeComponent::WeekYear(v) => *v as u32,
        }
    }
}
