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


pub fn stop() {
    server::stop_all_jobs();
}
