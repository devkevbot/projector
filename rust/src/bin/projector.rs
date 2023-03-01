use anyhow::Result;
use clap::Parser;
use projector::config::{Config, Operation};
use projector::opts::Opts;
use projector::projector::Projector;

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut proj: Projector = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value)?;
            println!("{}", value)
        }
        Operation::Print(Some(key)) => {
            if let Some(value) = proj.get_value(&key) {
                println!("{}", value);
            }
        }
        Operation::Add(key, value) => {
            proj.set_value(&key, &value);
            proj.save()?;
        }
        Operation::Remove(key) => {
            proj.remove_value(&key);
            proj.save()?;
        }
    }
    Ok(())
}
