#[macro_use]
extern crate clap;

use std::env;
use clap::{App, Arg, SubCommand, AppSettings};

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
    let app = App::new("enviriusx")
        .setting(AppSettings::SubcommandRequired)
        .about("Universal Virtual Environment Manager")
        .version(crate_version!())
        .version_short("v");

    let m = app.subcommand(
            SubCommand::with_name("ls")
                .about("List environments")
                .arg(
                    Arg::with_name("no-meta")
                ))
        .get_matches();

    let args: Vec<_> = env::args().collect();
    // cmd::run(args[1].as_ref(), &nv, &args[2..]);
    match m.subcommand_name() {
        Some(cmd)   => cmd::run(cmd, &nv, &args[2..]),
        None        => println!("Wrong command"),
    }
}
