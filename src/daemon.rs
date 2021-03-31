use std::io::Read;

use chrono::{DateTime, Duration, Utc};
use unix_socket::UnixListener;
use users::get_current_uid;

use crate::event::{EventTrigger, Operation};

pub fn daemon_main() {
    let listen_t = std::thread::spawn(|| daemon_socket_listen());
    let event_dispatcher_t = std::thread::spawn(|| daemon_event_dispatcher());
    listen_t.join().expect("Could not wait for listener thread to exit");
    event_dispatcher_t.join().expect("Could not wait for event dispatcher thread to exit");
}

pub fn daemon_event_dispatcher() {
    

}

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
        println!("{:?}", op)
    }
}

impl EventTrigger {
    pub fn next_match(&self, after: DateTime<Utc>) -> Option<Duration> {
        match self {
            EventTrigger::Never => None,
            EventTrigger::Always => Some(Duration::seconds(0)),
            EventTrigger::Is(c) => todo!(),
            EventTrigger::Divisible(c) => todo!(),
            EventTrigger::OneOf(evts) => evts.iter().filter_map(|t| t.next_match(after)).min(),
            EventTrigger::AllOf(_) => todo!(),
        }
    }
}
