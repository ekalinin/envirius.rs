use common::{EnvHome};

static DESC: &'static str = "List environments";

static HELP: &'static str = "
Options:
  --no-meta    Do not show meta information of the environment
";

pub fn run(e: &EnvHome, argv: &[String]) -> () {
    if argv.len() > 0 && argv[0] == "--help" {
        usage();
        return;
    }
    e.print_environments()
}

pub fn usage() {
    println!("{}", DESC);
    println!("{}", HELP);
}
