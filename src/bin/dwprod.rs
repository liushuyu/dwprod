use clap::Command;
use std::process;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        process::exit(1)
    }
}

fn try_main() -> dwprod::Result<()> {
    let matches = parse_args();

    let opts = dwprod::Options::new(matches.get_one::<String>("file").unwrap());

    opts.producers(|producers| {
        while let Some(producer) = producers.next()? {
            println!("{}", producer);
        }

        Ok(())
    })?
}

fn parse_args() -> clap::ArgMatches {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(clap::Arg::new("file").required(true).help(
            "The shared library or executable we should search for \
             `DW_AT_producer` information in.",
        ))
        .get_matches()
}
