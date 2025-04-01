extern crate ansi_term;
extern crate rosc;
extern crate dirs;

use std::io::{self, Read};
use std::process;

mod file;
mod log_packet;
mod server;

/// Read code from STDIN and send to Sonic Pi Server.
///
pub fn eval_stdin() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    server::run_code(input);
}

/// Read code from a file and send to Sonic Pi Server.
///
pub fn eval_file(path: &str) {
    match file::read(path) {
        Ok(code) => server::run_code(code),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    }
}

/// Take some code and send to Sonic Pi Server.
///
pub fn eval(code: String) {
    server::run_code(code);
}

/// Check if something is listening on the Sonic Pi server's port.
/// If something is we can probably assume that it's the Sonic Pi Server,
/// so siginify this to the user.
///
pub fn check() {
    if server::server_port_in_use() {
        println!("Sonic Pi server listening on port {}", server::OSC_SERVER_PORT);
        process::exit(0);
    } else {
        println!("Sonic Pi server NOT listening on port {}", server::OSC_SERVER_PORT);
        process::exit(1);
    }
}

/// Instuct the Sonic Pi server to stop playing.
///
pub fn stop() {
    server::stop_all_jobs();
}

// TODO: Colour the word "error:"
const ADDR_IN_USE_MSG: &str =
    r#"error: Unable to listen for Sonic Pi server logs, address already in use.

This may because the Sonic Pi GUI is running and already listening on the desired port.
If the GUI is running this command cannot function, try running just the Sonic Pi server."#;

/// Print log messages sent by the Sonic Pi server.
/// This will fail if the GUI is running.
///
pub fn logs() {
    match server::follow_logs() {
        Err(server::FollowLogError::AddrInUse) => {
            println!("{}", ADDR_IN_USE_MSG);
            process::exit(1);
        }
        Err(server::FollowLogError::ReceiveFail(e)) => {
            println!("Unexpected error: {}\n", e);
            println!("Please report this error at https://github.com/lpil/sonic-pi-tool/issues");
            process::exit(1);
        }
        Ok(()) => (),
    };
}

/// Record the audio output of a Sonic Pi session to a local file.
/// Stop and save the recording when the <Enter> key is pressed.
///
pub fn record(path: &str) {
    server::start_recording();
    println!("Recording started, saving to {}", path);
    println!("Press Enter to stop the recording...");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            server::stop_and_save_recording(path.to_string());
        }
        Err(error) => {
            println!("error: {}", error);
            server::stop_and_save_recording(path.to_string());
        }
    }
}
