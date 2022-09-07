use crate::config::Config;
use anyhow::{anyhow, Result};
use clap::Args;
use mev_boost_rs::Service;
use mev_build_rs::Network;

#[derive(Debug, Args)]
#[clap(about = "🚀 connecting proposers to the external builder network")]
pub(crate) struct Command {
    #[clap(env, default_value = "config.toml")]
    config_file: String,
}

impl Command {
    pub(crate) async fn execute(&self, network: Network) -> Result<()> {
        let config_file = &self.config_file;

        let config = Config::from_toml_file(config_file)?;

        if let Some(config) = config.boost {
            Service::from(config, network).run().await;
            Ok(())
        } else {
            Err(anyhow!("missing boost config from file provided"))
        }
    }
}
