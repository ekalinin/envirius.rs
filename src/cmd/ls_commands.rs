use common::{Nv};

extern crate clap;
use clap::{App, SubCommand, ArgMatches};


pub fn run(_: &Nv, _: &ArgMatches) -> () {
    for c in super::get_commands() {
        println!("{}", c);
    }
}

pub fn get_command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("ls-commands")
        .about("List available commands")
}

