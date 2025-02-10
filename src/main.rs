use clap::{Arg, ArgAction, Command};
use signal_hook::{consts::TERM_SIGNALS, flag, iterator::Signals};
use std::{
    env,
    error::Error,
    sync::{
        atomic::AtomicBool,
        mpsc, Arc,
    },
    thread,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments.
    let _matches = Command::new("fscape")
        .version(env!("CARGO_PKG_VERSION")) // Uses package version from Cargo.toml
        .about("Delete files to control filesystem disk usage")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Use FILE as the configuration file.")
                .value_name("FILE")
                .env("FSCAPE_CONFIG") // Alternative source for config
        )
        .arg(
            Arg::new("daemon")
            .short('d')
            .long("daemon")
            .help("Run fscape in background monitoring mode.")
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("delete")
                .long("delete")
                .help("Enable deletion rules.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Increase logging verbosity.")
                .action(ArgAction::Count), // Supports multiple -v flags
        )
        .get_matches();

    // Make sure double CTRL+C and similar kills
    eprintln!("Registering termination signal handlers...");
    let term_now = Arc::new(AtomicBool::new(false));
    for term_signal in TERM_SIGNALS {
        // When terminated by a second term signal, exit with exit code 1.
        // This will do nothing the first time (because term_now is false).
        flag::register_conditional_shutdown(*term_signal, 1, Arc::clone(&term_now))?;
        // But this will "arm" the above for the second time, by setting it to true.
        // The order of registering these is important, if you put this one first, it will
        // first arm and then terminate ‒ all in the first round.
        flag::register(*term_signal, Arc::clone(&term_now))?;
    }

    // Read and process config file.
    // todo

    // Create a channel to notify the rule execution thread that a signal was
    // received.
    let (tx, rx) = mpsc::channel::<i32>();

    // Create an iterator over incoming termination signals. Our main thread will use this to wait
    // for a signal which it will then send to the rule execution thread.
    let mut term_signals = Signals::new(TERM_SIGNALS)?;
    let term_handle = term_signals.handle();

    // Start the rule execution thread.
    let exec_handle = thread::spawn(move || {
        for term_signal in rx {
            println!("Worker received a message: {:?} and term_now= {:?}", term_signal, term_now);
            break;
        }
        term_handle.close();
    });

    // Wait for a signal on the main thread by reading from the termination signal iterator.
    for term_signal in term_signals.forever() {
        println!("Received signal: {}", term_signal);
        tx.send(term_signal).ok();
    }

    exec_handle.join().ok();
    Ok(())
}


