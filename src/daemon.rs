use std::{io::Read, net::Shutdown, ops::Add};

use chrono::{DateTime, Duration, Utc};
use unix_socket::UnixListener;
use users::get_current_uid;

use crate::{
    event::{EventTrigger, Operation},
    time::Time,
};

pub fn daemon_main() {
    let listen_t = std::thread::spawn(|| daemon_socket_listen());
    let event_dispatcher_t = std::thread::spawn(|| daemon_event_dispatcher());
    listen_t
        .join()
        .expect("Could not wait for listener thread to exit");
    event_dispatcher_t
        .join()
        .expect("Could not wait for event dispatcher thread to exit");
}

pub fn daemon_event_dispatcher() {}

pub fn daemon_socket_listen() {
    let sock_path = format!("/run/user/{}/cald", get_current_uid());
    let sock = UnixListener::bind(sock_path).expect("Could not connect to socket.");
    loop {
        let (mut stream, addr) = sock.accept().expect("Could not accept client");
        println!("Client connected: {:?}", addr);
        let mut s = String::new();
        stream
            .read_to_string(&mut s)
            .expect("could not read string");
        let op: Operation = serde_json::from_str(s.as_str()).expect("Invalid JSON content");
        println!("{:?}", op);
        stream.shutdown(Shutdown::Both).unwrap();
    }
}

impl EventTrigger {
    pub fn next_match(&self, after: &Time) -> Option<Time> {
        match self {
            EventTrigger::Never => None,
            EventTrigger::Always => Some(Time(0)),
            EventTrigger::Is(c) => {
                //TODO make this work properly
                let components = after.get_components();
                let comp_max = components.max_of_component(&c);
                let comp = components.value_of_component(&c) as u64;
                let comp_trigger = c.get_value() as u64;

                let comp_needed = match (comp, comp_trigger, comp_max) {
                    (c, ct, _) if ct > c => Some(Time(ct - c)),
                    (c, ct, cm) if ct < c && cm.is_some() => {
                        Some(Time(cm.unwrap() as u64 - ct + c))
                    }
                    _ => None,
                };
                match comp_needed {
                    None => None,
                    Some(tn) => Some(Time(after.value() + tn.value())),
                }
            }
            EventTrigger::Divisible(c) => todo!(), // TODO implement divisibility rules
            EventTrigger::OneOf(evts) => evts.iter().filter_map(|t| t.next_match(&after)).min(),
            EventTrigger::AllOf(evts) => {
                //TODO make this effient by first evaluating the components with the longest interval
                let n = 10;
                let evals = evts.iter().map(|et| {
                    let time = after.clone();

                });

                None
            },
        }
    }
}
