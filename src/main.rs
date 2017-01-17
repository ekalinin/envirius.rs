use std::env;

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
    let args: Vec<_> = env::args().collect();
    cmd::run(args[1].as_ref(), &nv, &args[2..]);
}
