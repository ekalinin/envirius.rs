pub mod ls;

use common::{EnvHome};

pub fn run(cmd: &str, nv: &EnvHome, args: &[String]) -> () {
    match cmd {
        "ls" => ls::run(&nv, &args),
        _ => print_help(),
    }
}

fn print_help() -> () {
    println!("Help page will be here ... ")
}
