#[macro_use]
extern crate clap;

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

    let gm = app.get_matches();
    if let Some(cmd) = gm.subcommand_name() {
        if let Some(ref m) = gm.subcommand_matches(cmd) {
            cmd::run(cmd, &nv, &m);
        }
    }
}
