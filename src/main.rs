use chrono::Utc;

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate actix_web;

mod cli;
mod common;
mod config;
mod constants;
mod logging;
mod sql;
mod web;

lazy_static! {
    pub static ref CONFIG: config::Config = config::read_config().unwrap();
}
type Result<T> = anyhow::Result<T>;

const SERVICE_NAME: &str = "SPORD TRACKER";

#[actix_web::main]
async fn main() -> Result<()> {
    common::set_cwd_to_exe()?;

    if !config::check_config_exists()? {
        println!("Wrote default config to file, please edit it if necessary!");
        return Ok(());
    }

    sql::check_initialized().await?;

    cli::handle_cli().await?;
    logging::setup()?;

    web::start().await?;

    Ok(())
}

async fn test_spord_creation() -> Result<()> {
    let mut spord = sql::models::SpordRecord {
        id: 0,
        customer_name: "bob".to_string(),
        customer_email: Some("hello@example.com".to_string()),
        customer_phone: None,
        part: "TRA9780".to_string(),
        state: sql::models::SpordState::Ordered,
        creation_date: Utc::now(),
        received_date: None,
        comments: Some("test field".to_string()),
    };
    info!("spord: {:?}", spord);
    sql::spord_create(None, spord.clone()).await?;

    spord.customer_phone = Some("test phone number!".to_string());
    sql::spord_update(None, spord).await?;
    Ok(())
}

fn prompt_user_input(prompt: &str) -> std::io::Result<String> {
    use std::io;
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    if input.ends_with('\r') {
        input.pop();
    }
    if input.ends_with('\n') {
        input.pop();
    }

    Ok(input)
}
