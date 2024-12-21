use crate::config;
use clap::{Arg, ArgMatches, Command};

fn gen_clap() -> Command {
    let app = command!().arg(
        Arg::new("write-config")
            .long("write-config")
            .required(false)
            .num_args(1)
            .help("Write a default configuration"),
    );
    app
}

fn handle_matches(matches: ArgMatches) -> anyhow::Result<()> {
    if matches.contains_id("write-config") {
        config::write_default_config(crate::constants::CONFIG_LOCATION)?;
        println!("Wrote default config!");
        std::process::exit(0);
    }

    Ok(())
}

pub fn handle_cli() -> anyhow::Result<()> {
    let matches = gen_clap().get_matches();

    handle_matches(matches)?;

    Ok(())
}
