#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod cli;
mod config;
mod constants;
mod logging;
mod sql;
mod web;

lazy_static! {
    pub static ref CONFIG: config::Config = config::read_config().unwrap();
}
type Result<T> = anyhow::Result<T>;

fn set_cwd_to_exe() -> std::io::Result<()> {
    let mut path = std::env::current_exe()?;
    path.pop();

    std::env::set_current_dir(path)?;

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    set_cwd_to_exe()?;

    cli::handle_cli()?;
    logging::setup()?;

    sql::check_initialized().await?;

    web::start().await?;

    Ok(())
}
