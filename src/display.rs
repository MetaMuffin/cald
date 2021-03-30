use std::fmt::Display;

use crate::event::{Event, EventTrigger, TimeComponent};

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:\n\ttrigger: \n\t{}\n\ttags: {}",
            self.name,
            format!("\t{:#}", self.trigger).replace("\n", "\n\t"),
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
                "when one of the condition matches: {}",
                cs.iter()
                    .map(|e| format!("\t{:#}", e).replace("\n", "\n\t"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            EventTrigger::AllOf(cs) => format!(
                "when all condition match: {}",
                cs.iter()
                    .map(|e| format!("\t{:#}", e).replace("\n", "\n\t"))
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
        }
    }
}
