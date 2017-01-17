use common::{Nv};

static DESC: &'static str = "List environments";

static HELP: &'static str = "
Options:
  --no-meta    Do not show meta information of the environment
";

pub fn run(e: &Nv, argv: &[String]) -> () {
    let mut show_meta = true;
    if argv.len() > 0 {
        if argv[0] == "--help" {
            usage();
            return;
        }
        if argv[0] == "--no-meta" {
            show_meta = false;
        }
    }
    e.print_environments(show_meta);
}

pub fn usage() {
    println!("{}", DESC);
    println!("{}", HELP);
}
