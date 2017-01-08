use std::env;

mod common;

/*
mod cmd;
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

    // setup envs root
    let nv = common::EnvHome::new(nv_home);
    let envs = nv.list_environments();
    println!("Vector: {:?}", envs);
}
