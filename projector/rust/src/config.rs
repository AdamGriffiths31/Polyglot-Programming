use crate::opts::Opts;
use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

#[derive(Debug)]
pub struct Config {
    pub pwd: PathBuf,
    pub config: PathBuf,
    pub operation: Operation,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self, Self::Error> {
        let pwd = get_pwd(value.pwd)?;
        let config = get_config(value.config)?;
        let operation = value.args.try_into()?;

        return Ok(Config {
            pwd,
            config,
            operation,
        });
    }
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expected a term");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow::anyhow!("expected 2 arguments for add"));
            }

            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(drain.next().unwrap(), drain.next().unwrap()));
        }

        if term == "remove" {
            if value.len() != 2 {
                return Err(anyhow::anyhow!("expected 1 argument for remove"));
            }

            let arg = value.pop().expect("expected an argument");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow::anyhow!("expected 0 or 1 arguments"));
        }

        let arg = value.pop().expect("expected an argument");
        Ok(Operation::Print(Some(arg)))
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = config {
        return Ok(path);
    }

    // XDG_CONFIG_HOME or ~/.config
    let loc = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let mut home = std::env::var("HOME").expect("missing HOME");
        home.push_str("/.config");
        home
    });
    let mut loc = PathBuf::from(loc);
    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = pwd {
        return Ok(path);
    }

    let loc = std::env::current_dir().context("missing current directory")?;
    return Ok(loc);
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::{config::Operation, opts::Opts};
    use anyhow::Result;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            pwd: None,
            config: None,
            args: vec![],
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));
        return Ok(());
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Opts {
            pwd: None,
            config: None,
            args: vec!["foo".into()],
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some("foo".into())));
        return Ok(());
    }

    #[test]
    fn test_add_key() -> Result<()> {
        let opts: Config = Opts {
            pwd: None,
            config: None,
            args: vec![
                String::from("add"),
                String::from("foo"),
                String::from("bar"),
            ],
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Add("foo".into(), "bar".into()));
        return Ok(());
    }

    #[test]
    fn test_remove_key() -> Result<()> {
        let opts: Config = Opts {
            pwd: None,
            config: None,
            args: vec![String::from("remove"), String::from("foo")],
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Remove("foo".into()));
        return Ok(());
    }
}
