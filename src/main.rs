use std::io::{Read, Write};

use cald::event::{Event, EventFilter};
use unix_socket::{UnixListener, UnixStream};
use users::get_current_uid;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    client_main(args);
}

pub enum Operation {
    Create(Event),
    Remove(EventFilter),
    Update(EventFilter, Event),
    Query(EventFilter),
}

pub fn daemon_main() {
    let sock_path = format!("/run/user/{}/cald", get_current_uid());
    let sock = UnixListener::bind(sock_path).expect("Could not connect to socket.");
    loop {
        let (mut stream, addr) = sock.accept().expect("Could not accept client");
        println!("Client connected: {:?}", addr);
        let mut s = String::new();
        stream.read_to_string(&mut s).expect("could not read string");
        println!("{}", s)
    }
}

pub fn client_main(mut args: Vec<String>) {
    while args.len() > 0 {
        let ax = args.pop().expect("Du bist ein kek");
        if ax.starts_with("-") {
            let mut aflags = vec![];
            if ax.starts_with("--") {
                aflags.push(ax[1..].to_string())
            } else {
                aflags.extend(ax.chars().map(|c| String::from(c)))
            }
            for aflag in aflags {
                match aflag.as_str() {
                    "c" | "-create" | "-create-event" => {}
                    "r" | "-remove" | "-remove-event" => {}
                    "u" | "-update" | "-update-event" => {}
                    "q" | "-query" | "-query-event" => {}
                    "d" | "-daemon" | "-deamon" => daemon_main(),
                    _ => {
                        println!()
                    }
                }
            }
        }
    }
    let sock_path = format!("/run/user/{}/cald", get_current_uid());
    let mut sock = UnixStream::connect(sock_path).expect("Could not connect to socket.");
    sock.write_fmt(format_args!("Kek!")).unwrap();
}
