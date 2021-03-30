use cald::cli::cli_main;

fn main() {
    std::panic::set_hook(Box::new(|p| {
        eprint!("\x1b[1;31m");
        eprintln!("A critical cald error was thrown. Here is a some panic info for you:");
        eprintln!("{:#}", p);
        eprintln!("Use '--help' to get help or, even better, run 'man cald'.");
        eprint!("\x1b[0m")
    }));

    let args = std::env::args().collect::<Vec<_>>();
    cli_main(args);
}
