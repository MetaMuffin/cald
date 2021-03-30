use std::{
    collections::HashMap,
    io::{Read, Write},
};

use cald::{cli::cli_main, event::{Event, EventFilter, EventTrigger, Operation, TimeComponent}};
use unix_socket::{UnixListener, UnixStream};
use users::get_current_uid;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    cli_main(args);
}

