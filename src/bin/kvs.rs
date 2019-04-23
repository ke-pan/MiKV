use clap::{App, Arg, SubCommand};

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("get value")
                .arg(Arg::with_name("key").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("set key value")
                .arg(Arg::with_name("key").required(true).index(1))
                .arg(Arg::with_name("value").required(true).index(2)),
        )
        .get_matches();

    match args.subcommand() {
        ("get", Some(sub_m)) => {
            panic!("unimplemented");
        }
        ("set", Some(sub_m)) => {
            panic!("unimplemented");
        }
        _ => unreachable!()
    }
}
