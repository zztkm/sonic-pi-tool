#[macro_use]
extern crate clap;
use clap::{App, AppSettings, SubCommand};

extern crate lib;

fn main() {
    let cli_app = App::new("sonic-pi-tool")
        .author("Louis Pilfold <louis@lpil.uk>")
        .setting(AppSettings::SubcommandRequired)
        .version(crate_version!());

    let eval_stdin = SubCommand::with_name("eval-stdin")
        .about("Reads Sonic Pi code from stdin and sends it to the server");

    let stop = SubCommand::with_name("stop")
        .about("Stops all currently playing music on the server");

    let matches = cli_app.subcommand(eval_stdin)
        .subcommand(stop)
        .get_matches();

    match matches.subcommand_name() {
        Some("eval-stdin") => lib::eval_stdin(),
        Some("stop") => lib::stop(),
        _ => panic!("This should be unreachable."),
    }
}
