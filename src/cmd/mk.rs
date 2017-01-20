use common::{Nv, Lang};

extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};


pub fn run(e: &Nv, args: &ArgMatches) -> () {
    println!("Create environment ...");
    println!("  On:     {}", args.is_present("on"));
    println!("  Force:  {}", args.is_present("force"));
    println!("  Name:   {}", args.value_of("name").unwrap_or("undefined"));
    println!("  Plugins:{:?}", args.values_of("plugins").unwrap().collect::<Vec<_>>());
    for p in args.values_of("plugins").unwrap().collect::<Vec<_>>() {
        println!(" * {:?}", Lang::from(p));
    }
}

pub fn get_command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("mk")
        .about("Create environment")
        .arg(
            Arg::with_name("on")
                .long("on")
                .help("Activate environment after installation")
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("Re-create environment if it already exists")
        )
        .arg(
            Arg::with_name("name")
                .help("Environment name")
                .required(true)
        )
        .arg(
            Arg::with_name("plugins")
                .help("List of plugins needed in the environment")
                .multiple(true)
        )
}

