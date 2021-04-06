use std::{collections::HashMap, io::Write};

use unix_socket::UnixStream;
use users::get_current_uid;

use crate::{
    daemon::daemon_main,
    display::ident,
    event::TimeComponent,
    event::{Event, EventTrigger, Operation},
};

pub fn cli_main(mut args: Vec<String>) {
    let mut op = Operation::None;
    while args.len() > 0 {
        let ax = args.remove(0);
        if ax.starts_with("-") {
            let mut aflags = vec![];
            if ax.starts_with("--") {
                aflags.push(ax[1..].to_string())
            } else {
                aflags.extend(ax.chars().map(|c| String::from(c)))
            }
            for aflag in aflags {
                match aflag.as_str() {
                    "n" | "-name" | "-message" => match &mut op {
                        Operation::Create(ev) => {
                            ev.name = args.remove(0);
                        }
                        _ => panic!("the message option makes sense for this mode"),
                    },
                    "t" | "-trigger" => match &mut op {
                        Operation::Create(ev) => {
                            let trigger = parse_event_trigger(args.remove(0).as_str());
                            match &mut ev.trigger {
                                EventTrigger::Never => ev.trigger = trigger,
                                EventTrigger::OneOf(ets) => ets.push(trigger),
                                et => ev.trigger = EventTrigger::OneOf(vec![et.clone(), trigger]),
                            }
                        }
                        _ => panic!("the trigger option makes sense for this mode"),
                    },
                    "d" | "-data" => {}
                    "g" | "-tags" => {}

                    "C" | "-create" | "-create-event" => {
                        op = Operation::Create(Event {
                            data: HashMap::new(),
                            name: String::from("<unnamed>"),
                            trigger: EventTrigger::Never,
                            tags: vec![],
                        })
                    }
                    "R" | "-remove" | "-remove-event" => {}
                    "U" | "-update" | "-update-event" => {}
                    "Q" | "-query" | "-query-event" => {}
                    "D" | "-daemon" | "-deamon" => daemon_main(),
                    _ => eprintln!("Ignoring unknown flag: {}", aflag),
                }
            }
        }
    }
    println!("committing an operation:\n{}", ident(format!("{:#}", op)));
    let sock_path = format!("/run/user/{}/cald", get_current_uid());
    let mut sock = UnixStream::connect(sock_path)
        .expect("Could not connect to socket, maybe the daemon is not running.");
    sock.write_fmt(format_args!(
        "{}",
        serde_json::to_string(&op).expect("Could not serialize operation")
    ))
    .expect("Could not send data over the socket");
}

pub fn parse_event_trigger(s: &str) -> EventTrigger {
    let mut trigger = EventTrigger::Never;
    let mut buf_num = None;
    let mut buf_divisible_mode = false;
    let mut buf_all_of_mode = false;
    let mut buf_params: Option<Vec<String>> = None;
    let mut buf_bracket_level = 0;
    for c in s.chars() {
        buf_bracket_level += match c {
            '(' | '[' => 1,
            ')' | ']' => -1,
            _ => 0,
        };
        if buf_bracket_level != 0 {
            if let Some(s) = &mut buf_params {
                let i = s.len() - 1;
                s[i].push(c);
            }
            if c != ',' && c != ')' && c != ']' && c != '(' && c != '[' {
                continue;
            }
            if buf_bracket_level != 1 {
                continue;
            }
        }
        match c {
            ',' => {
                if buf_bracket_level == 1 {
                    match &mut buf_params {
                        Some(p) => p.push(String::new()),
                        None => panic!("Unexpected ','"),
                    }
                }
            }
            '(' | '[' => {
                buf_params = Some(vec![String::new()]);
                buf_all_of_mode = c == '(';
            }
            ')' | ']' => {
                if (c == ')') != buf_all_of_mode {
                    eprintln!("deprecation warning: use of different bracket types")
                }
                let params = buf_params
                    .clone()
                    .expect("')' and ']' is only allowed after the corresponding '(' or '['")
                    .iter()
                    .map(|s| parse_event_trigger(s.as_str()))
                    .collect::<Vec<_>>();
                trigger = match buf_all_of_mode {
                    true => EventTrigger::AllOf(params),
                    false => EventTrigger::OneOf(params),
                }
            }
            '0'..='9' => match buf_num {
                Some(n) => buf_num = Some(n * 10 + format!("{}", c).parse::<u32>().unwrap()),
                None => buf_num = Some(format!("{}", c).parse::<u32>().unwrap()),
            },
            '-' => trigger = EventTrigger::Never,
            '%' => buf_divisible_mode = true,
            'y' | 'M' | 'w' | 'D' | 'd' | 'h' | 'm' | 's' => {
                if let Some(value) = buf_num {
                    let component = TimeComponent::from_unit_and_value(c, value);
                    let new_trigger = match buf_divisible_mode {
                        true => EventTrigger::Divisible(component),
                        false => EventTrigger::Is(component),
                    };
                    match &mut trigger {
                        EventTrigger::Never => trigger = new_trigger,
                        EventTrigger::AllOf(e) => e.push(new_trigger),
                        _ => trigger = EventTrigger::AllOf(vec![trigger, new_trigger]),
                    }
                    buf_num = None;
                    buf_divisible_mode = false;
                } else {
                    panic!("No number before the time unit")
                }
            }
            _ => panic!("While parsing trigger literal an unexpected char was found"),
        }
    }
    return trigger;
}

impl TimeComponent {
    pub fn from_unit_and_value(unit: char, value: u32) -> Self {
        match unit {
            'y' => TimeComponent::Years(value),
            'M' => TimeComponent::MonthYear(value as u8),
            'w' => TimeComponent::WeekYear(value as u8),
            'D' => TimeComponent::DayWeek(value as u8),
            'd' => TimeComponent::DayMonth(value as u8),
            'h' => TimeComponent::HourDay(value as u8),
            'm' => TimeComponent::MinuteHour(value as u8),
            's' => TimeComponent::SecondMinute(value as u8),
            _ => panic!("Unexpected time component unit"),
        }
    }
}
