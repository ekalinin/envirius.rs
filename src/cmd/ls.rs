use common::{EnvHome};

pub fn run(e: &EnvHome, _: &[String]) -> () {
    e.print_environments()
}