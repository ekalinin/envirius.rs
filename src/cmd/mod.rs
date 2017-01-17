pub mod ls;

use common::{Nv};

pub fn run(cmd: &str, nv: &Nv, args: &[String]) -> () {
    match cmd {
        "ls" => ls::run(&nv, &args),
        _ => print_help(),
    }
}

fn print_help() -> () {
    println!("Help page will be here ... ")
}
