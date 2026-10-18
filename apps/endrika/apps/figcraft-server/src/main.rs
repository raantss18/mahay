//! `figcraft-server`: see `docs/server.md`, or run it with `--help`.

use std::process::ExitCode;

use figcraft_server::cli::{self, Command};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cli = match cli::parse(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("figcraft-server: {e}\n(figcraft-server --help lists what it takes)");
            return ExitCode::from(2);
        }
    };
    if cli.command != Command::Serve {
        return match cli::run(&cli) {
            Ok(out) => {
                print!("{out}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("figcraft-server: {e}");
                ExitCode::FAILURE
            }
        };
    }
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).target(env_logger::Target::Stdout).init();
    let mut running = match figcraft_server::start(&cli.config) {
        Ok(r) => r,
        Err(e) => {
            log::error!("{e}");
            return ExitCode::FAILURE;
        }
    };
    // Serves until SIGTERM, SIGINT or SIGHUP (Ctrl+C or closing the console on Windows), then stops
    // gracefully: open files are saved and compacted, and everyone is told the server is
    // restarting (their apps reconnect once it runs again).
    let (stop, stopped) = std::sync::mpsc::channel();
    if let Err(e) = ctrlc::set_handler(move || {
        let _ = stop.send(());
    }) {
        // Still safe to kill: what was confirmed is on disk already.
        log::warn!("can't watch for stop signals ({e}); stopping won't save open files first");
        loop {
            std::thread::park();
        }
    }
    let _ = stopped.recv();
    log::info!("stopping: saving open files and telling everyone the server is restarting");
    running.stop();
    log::info!("stopped");
    ExitCode::SUCCESS
}
