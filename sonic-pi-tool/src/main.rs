#[macro_use]
extern crate clap;

use clap::{App, AppSettings, SubCommand};

fn main() {
    let cli_app = App::new("sonic-pi-tool")
        .author("Louis Pilfold <louis@lpil.uk>")
        .setting(AppSettings::SubcommandRequired)
        .version(crate_version!());

    let add_subcommand = SubCommand::with_name("eval")
        .about("Takes Sonic Pi code from STDIN and sends it to the server");

    let matches = cli_app.subcommand(add_subcommand).get_matches();

    match matches.subcommand_name() {
        Some("eval") => println!("Eval time!"),
        _ => panic!("This should be unreachable."),
    }
}
