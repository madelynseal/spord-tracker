use crate::config;
use clap::{Arg, ArgMatches, Command};

fn gen_clap() -> Command {
    let app = command!()
        .arg(
            Arg::new("write-config")
                .long("write-config")
                .required(false)
                .help("Write a default configuration"),
        )
        .arg(
            Arg::new("create-user")
                .long("create-user")
                .required(false)
                .num_args(0..=1)
                .help("Create a new user in the database"),
        );
    app
}

async fn handle_matches(matches: ArgMatches) -> anyhow::Result<()> {
    if matches.contains_id("write-config") {
        config::write_default_config(crate::constants::CONFIG_LOCATION)?;
        println!("Wrote default config!");
        std::process::exit(0);
    }

    if matches.contains_id("create-user") {
        let username = if let Some(username) = matches.get_one::<String>("create-user") {
            Some(username.to_owned())
        } else {
            None
        };

        crate::sql::user_create_console(None, username).await?;
    }

    Ok(())
}

pub async fn handle_cli() -> anyhow::Result<()> {
    let matches = gen_clap().get_matches();

    handle_matches(matches).await?;

    Ok(())
}
