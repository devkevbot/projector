use anyhow::Result;
use clap::Parser;
use projector::config::Config;
use projector::opts::Opts;

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    println!("{:?}", config);

    Ok(())
}