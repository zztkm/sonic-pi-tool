#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand};

extern crate lib;

fn main() {
    let cli_app = App::new("sonic-pi-tool")
        .author("Louis Pilfold <louis@lpil.uk>")
        .setting(AppSettings::SubcommandRequired)
        .version(crate_version!());

    let eval_stdin_cmd = SubCommand::with_name("eval-stdin")
        .about("Reads Sonic Pi code from stdin and sends it to the server");

    let matches = cli_app.subcommand(eval_stdin_cmd)
        .get_matches();

    match matches.subcommand_name() {
        Some("eval-stdin") => lib::eval_stdin(),
        _ => panic!("This should be unreachable."),
    }
}
