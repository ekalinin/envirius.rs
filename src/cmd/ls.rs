use common::{Nv};

extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};


pub fn run(e: &Nv, args: &ArgMatches) -> () {
    let mut show_meta = true;
    if args.is_present("no-meta") {
        show_meta = false;
    }
    e.print_environments(show_meta);
}

pub fn get_command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("ls")
        .about("List environments")
        .arg(
            Arg::with_name("no-meta")
                .long("no-meta")
                .help("Do not show environment's meta information")
        )
}

