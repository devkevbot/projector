use crate::opts::Opts;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Ok, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        Ok(Self {
            operation,
            pwd,
            config,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.is_empty() {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "add expects 2 arguments, but got {}",
                    value.len() - 1
                ));
            }

            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "rm expects 1 argument, but got {}",
                    value.len() - 1
                ));
            }

            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow!(
                "print expects 0 or 1 arguments, but got {}",
                value.len() - 1
            ));
        }

        let arg = value.pop().expect("to exist");
        Ok(Operation::Print(Some(arg)))
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let location = std::env::var("HOME").context("unable to get HOME")?;
    let mut location = PathBuf::from(location);

    location.push("projector");
    location.push("projector.json");

    Ok(location)
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = pwd {
        return Ok(v);
    }

    let pwd = std::env::current_dir().context("error getting current_dir")?;
    Ok(pwd)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print_all() -> Result<()> {
        let config: Config = Opts {
            args: vec![],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(config.operation, Operation::Print(None));

        Ok(())
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let config: Config = Opts {
            args: vec!["foo".to_string()],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(config.operation, Operation::Print(Some("foo".to_string())));

        Ok(())
    }

    #[test]
    fn test_add_key_value() -> Result<()> {
        let config: Config = Opts {
            args: vec!["add".to_string(), "foo".to_string(), "bar".to_string()],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(
            config.operation,
            Operation::Add("foo".to_string(), "bar".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_remove_key_value() -> Result<()> {
        let config: Config = Opts {
            args: vec!["rm".to_string(), "foo".to_string()],
            config: None,
            pwd: None,
        }
        .try_into()?;

        assert_eq!(config.operation, Operation::Remove("foo".to_string()));

        Ok(())
    }
}
