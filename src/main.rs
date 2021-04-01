extern crate lazy_static;
use cald::{cli::cli_main, database::save_db};

fn main() {
    ctrlc::set_handler(|| {
        eprintln!("Received ctrl-c. Shutting down.");
        save_db();
        eprintln!("Bye");
        std::process::exit(0);
    })
    .or_else(|_| -> Result<(), ()> {
        eprintln!(
            "Could not set ctrl-c handler, database will not be saved if ctrl-c is received."
        );
        Ok(())
    })
    .unwrap();

    // std::panic::set_hook(Box::new(|p| {
    //     eprint!("\x1b[1;31m");
    //     eprintln!("A critical cald error was thrown. Here is a some panic info for you:");
    //     eprintln!("{:#}", p);
    //     eprintln!("Use '--help' to get help or, even better, run 'man cald'.");
    //     eprint!("\x1b[0m")
    // }));

    let args = std::env::args().collect::<Vec<_>>();
    cli_main(args);
}
