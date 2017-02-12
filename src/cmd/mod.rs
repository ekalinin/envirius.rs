extern crate clap;

use clap::{App, ArgMatches};
use common::Nv;


pub mod mk;
pub mod ls;
pub mod ls_commands;

type CmdResult = Result<(), i32>;

pub enum Command {
    Ls,
    LsCommands,
    Mk,
}

fn from_str(cmd: &str) -> Command {
    match cmd {
        "ls" => Command::Ls,
        "ls-commands" => Command::LsCommands,
        "mk" => Command::Mk,
        _ => panic!("Wrong command!"),
    }
}

pub fn to_str<'a>(cmd: Command) -> &'a str {
    match cmd {
        Command::Ls => "ls",
        Command::LsCommands => "ls-commands",
        Command::Mk => "mk",
    }
}

pub fn run(cmd: &str, nv: &Nv, args: &ArgMatches) -> CmdResult {
    match from_str(cmd) {
        Command::Ls => ls::run(nv, args),
        Command::LsCommands => ls_commands::run(nv, args),
        Command::Mk => mk::run(nv, args),
    }
}

pub fn get_commands() -> Vec<Command> {
    vec![Command::Ls, Command::LsCommands, Command::Mk]
}

pub fn get_command<'a>(cmd: Command) -> App<'a, 'a> {
    match cmd {
        Command::Ls => ls::get_command(),
        Command::LsCommands => ls_commands::get_command(),
        Command::Mk => mk::get_command(),
    }
}