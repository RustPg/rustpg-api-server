extern crate clap;
extern crate rustc_serialize;

mod errors;

use clap::{App, AppSettings, SubCommand, Arg};

fn main() {
    let menu = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("run")
                        .about("Run server api")
                        .arg(Arg::with_name("config")
                                 .short("c")
                                 .long("config")
                                 .help("Path to config for server api")
                                 .takes_value(true)))
        .get_matches();

    match menu.subcommand() {
        ("run", Some(cmd)) => {
            // todo
        }
        _ => {
            println!("{}", menu.usage());
        }
    }
}
