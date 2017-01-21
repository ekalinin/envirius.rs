extern crate clap;

use clap::{App, ArgMatches};
use common::{Nv};


pub mod mk;
pub mod ls;
pub mod ls_commands;

type CmdResult = Result<(), i32>;

pub fn run(cmd: &str, nv: &Nv, args: &ArgMatches) -> CmdResult {
    match cmd {
        "ls" => ls::run(nv, args),
        "ls-commands" => ls_commands::run(nv, args),
        "mk" => mk::run(nv, args),
        _ => panic!("Wrong command!"),
    }
}

pub fn get_commands<'a>() -> Vec<&'a str> {
    vec!["ls", "ls-commands", "mk"]
}

pub fn get_command(cmd: &str) -> App {
    match cmd {
        "ls" => ls::get_command(),
        "ls-commands" => ls_commands::get_command(),
        "mk" => mk::get_command(),
        _ => panic!("Wrong command!"),
    }
}