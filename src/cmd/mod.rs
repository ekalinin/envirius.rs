extern crate clap;

use clap::{App, ArgMatches};
use common::{Nv};


pub mod ls;


pub fn run(cmd: &str, nv: &Nv, args: &ArgMatches) -> () {
    match cmd {
        "ls" => ls::run(nv, args),
        _ => panic!("Wrong command!"),
    }
}

pub fn get_commands<'a>() -> Vec<&'a str> {
    vec!["ls"]
}

pub fn get_command(cmd: &str) -> App {
    match cmd {
        "ls" => ls::get_command(),
        _ => panic!("Wrong command!"),
    }
}