use std::io::Read;

use unix_socket::UnixListener;
use users::get_current_uid;

use crate::event::Operation;


pub fn daemon_main() {
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
