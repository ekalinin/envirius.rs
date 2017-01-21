#[macro_use]
extern crate clap;

use std::process;
use std::env;
use clap::{App, AppSettings};

mod common;
mod cmd;

/*
mod plugins;
*/

fn main() {
    // detecting home directory
    let nv_home = match env::var("NV_HOME") {
        Ok(val) => val,
        _ => match env::var("HOME") {
            Ok(val) => val + "/.envirius",
            _ => String::from("/opt/envirius")
        },
    };

    // setup root
    let nv = common::Nv::new(nv_home);

    // cli args
    let mut app = App::new("enviriusx")
        .setting(AppSettings::SubcommandRequired)
        .about("Universal Virtual Environment Manager")
        .version(crate_version!())
        .version_short("v");

    for c in cmd::get_commands() {
        app = app.subcommand(cmd::get_command(c))
    }

    let matches = app.get_matches();
    if let Some(cmd_name) = matches.subcommand_name() {
        if let Some(ref cmd_args) = matches.subcommand_matches(cmd_name) {
            let status = match cmd::run(cmd_name, &nv, &cmd_args) {
                Ok(_) => 0,
                Err(code) => code,
            };
            process::exit(status);
        }
    }
}
