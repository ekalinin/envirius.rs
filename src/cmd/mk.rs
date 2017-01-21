use common::{Nv, Lang};

extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};


pub fn run(e: &Nv, args: &ArgMatches) -> super::CmdResult {
    println!("Create environment ...");
    println!("  On:     {}", args.is_present("on"));
    println!("  Force:  {}", args.is_present("force"));
    println!("  Name:   {}", args.value_of("name").unwrap_or("undefined"));
    println!("  Plugins:{:?}", args.values_of("plugins").unwrap().collect::<Vec<_>>());
    for p in args.values_of("plugins").unwrap().collect::<Vec<_>>() {
        println!(" * {:?}", Lang::from(p));
    }

    let env_name = args.value_of("name").expect("environment's name is null");
    if e.is_exists(env_name) {
        if args.is_present("force") {
            e.remove_env(env_name);
        } else {
            println!("Environment with name '{}' is already exists.", env_name);
            println!("Please, choose another name and try again.");
            println!("Or use --force option.");
            return Err(1);
        }
    } else {
        let langs = args.values_of("plugins")
            .unwrap().collect::<Vec<_>>().iter()
            .map(|p| Lang::from(p))
            .collect::<Vec<_>>();
        e.create_env(env_name, langs);
    }

    Ok(())
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
                .help("List of plugins needed in the environment (example: go=1.7.3)")
                .multiple(true)
                .required(true)
        )
}

