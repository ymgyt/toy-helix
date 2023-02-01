use anyhow::{Error, Result};

use crate::config::Config;

pub struct Application {
    config: Config,
}

impl Application {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    pub async fn run(&mut self) -> Result<i32, Error> {
        Ok(0)
    }
}
