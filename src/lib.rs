extern crate rosc;

use std::process;

mod server;
mod file;

use std::io::{self, Read};

/// Read code from STDIN and send to Sonic Pi Server
///
pub fn eval_stdin() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    server::run_code(input);
}


/// Read code from a file and send to Sonic Pi Server
///
pub fn eval_file(path: String) {
    match file::read(path) {
        Ok(code) => server::run_code(code),
        Err(msg) => {
            println!("{}", msg);
            process::exit(1);
        }
    }
}

/// Take some code and send to Sonic Pi Server
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
        println!("Sonic Pi server listening on port 4557");
        process::exit(0);
    } else {
        println!("Sonic Pi server NOT listening on port 4557");
        process::exit(1);
    }
}


/// Instuct the Sonic Pi server to stop playing.
///
pub fn stop() {
    server::stop_all_jobs();
}
