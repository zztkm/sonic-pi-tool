#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

extern crate lib;

fn main() {
    let cli_app = App::new("sonic-pi-tool")
        .author("Louis Pilfold <louis@lpil.uk>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!());

    let check = SubCommand::with_name("check")
        .about("Check if the Sonic Pi server is listening on port 4560");

    let eval = SubCommand::with_name("eval")
        .about("Takes a string of Sonic Pi code and sends it to the server")
        .arg(
            Arg::with_name("TOKEN")
                .help("A token to identify this client")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("CODE")
                .help("A string of Sonic Pi code")
                .required(true)
                .index(2),
        );

    let eval_stdin = SubCommand::with_name("eval-stdin")
        .about("Reads Sonic Pi code from stdin and sends it to the server")
        .arg(
            Arg::with_name("TOKEN")
                .help("A token to identify this client")
                .required(true)
                .index(1)
        );


    let eval_file = SubCommand::with_name("eval-file")
        .about("Reads Sonic Pi code from a file and sends it to the server")
        .arg(
            Arg::with_name("TOKEN")
                .help("A token to identify this client")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("PATH")
                .help("Path to the file of Sonic Pi code")
                .required(true)
                .index(2),
        );

    let stop =
        SubCommand::with_name("stop").about("Stops all currently playing music on the server");

    let logs =
        SubCommand::with_name("logs").about("Print log events emitted by the Sonic Pi server");

    let record = SubCommand::with_name("record")
        .about("Record the audio output of a Sonic Pi session")
        .arg(
            Arg::with_name("PATH")
                .help("Absolute path to the output file")
                .required(true)
                .index(1),
        );

    let matches = cli_app
        .subcommand(stop)
        .subcommand(check)
        .subcommand(logs)
        .subcommand(eval)
        .subcommand(eval_stdin)
        .subcommand(eval_file)
        .subcommand(record)
        .get_matches();

    match matches.subcommand_name() {
        Some("stop") => lib::stop(),
        Some("check") => lib::check(),
        Some("eval") => do_eval(&matches),
        Some("eval-file") => do_eval_file(&matches),
        Some("eval-stdin") => do_eval_stdin(&matches),
        Some("logs") => lib::logs(),
        Some("record") => do_record(&matches),
        _ => panic!("Unrecognised subcommand"), // This _should_ be unreachable
    }
}

fn do_eval_stdin(matches: &clap::ArgMatches) {
    let token = matches
        .subcommand_matches("eval")
        .unwrap()
        .value_of("TOKEN")
        .unwrap()
        .to_string();
    let token_i32 = token.parse::<i32>().unwrap();
    lib::eval_stdin(token_i32);
}

fn do_eval_file(matches: &clap::ArgMatches) {
    let token = matches
        .subcommand_matches("eval")
        .unwrap()
        .value_of("TOKEN")
        .unwrap()
        .to_string();
    let path = matches
        .subcommand_matches("eval-file")
        .unwrap()
        .value_of("PATH")
        .unwrap()
        .to_string();
    let token_i32 = token.parse::<i32>().unwrap();
    lib::eval_file(token_i32, &path);
}

fn do_eval(matches: &clap::ArgMatches) {
    let token = matches
        .subcommand_matches("eval")
        .unwrap()
        .value_of("TOKEN")
        .unwrap()
        .to_string();
    let code = matches
        .subcommand_matches("eval")
        .unwrap()
        .value_of("CODE")
        .unwrap()
        .to_string();
    let token_i32 = token.parse::<i32>().unwrap();
    lib::eval(token_i32, code);
}

fn do_record(matches: &clap::ArgMatches) {
    let path = matches
        .subcommand_matches("record")
        .unwrap()
        .value_of("PATH")
        .unwrap()
        .to_string();
    lib::record(&path);
}
