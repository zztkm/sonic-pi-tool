extern crate rosc;

mod server;

use std::io::{self, Read};

pub fn eval_stdin() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    server::run_code(input);
}
