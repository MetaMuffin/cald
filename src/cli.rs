use std::{collections::HashMap, io::Write};

use unix_socket::UnixStream;
use users::get_current_uid;

use crate::{
    daemon::daemon_main,
    display::ident,
    event::{Event, EventTrigger, Operation},
};

pub fn cli_main(mut args: Vec<String>) {
    let mut op = Operation::None;
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
                    "n" | "-name" | "-message" => match &mut op {
                        Operation::Create(ev) => {
                            ev.name = args.pop().expect("No args left to use for message")
                        }
                        _ => panic!("the message option makes sense for this mode"),
                    },
                    "t" | "-trigger" => {}
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
                    _ => println!(),
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
