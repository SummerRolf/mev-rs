use crate::builder_api_server::Server as ApiServer;
use crate::relay::Relay;
use crate::relay_mux::RelayMux;
use beacon_api_client::Client;
use futures::future::join_all;
use std::net::Ipv4Addr;
use url::Url;

#[derive(Debug)]
pub struct ServiceConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub relays: Vec<Url>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".parse().unwrap(),
            port: 18550,
            relays: vec![],
        }
    }
}

pub struct Service {
    config: ServiceConfig,
}

impl Service {
    pub fn from(config: ServiceConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) {
        let relays = self
            .config
            .relays
            .iter()
            .cloned()
            .map(|endpoint| Relay::new(Client::new(endpoint)));
        let relay_mux = RelayMux::new(relays);

        let mut tasks = vec![];

        let relay_mux_clone = relay_mux.clone();
        tasks.push(tokio::spawn(async move {
            relay_mux_clone.run().await;
        }));

        let builder_api = ApiServer::new(self.config.host, self.config.port, relay_mux);
        tasks.push(tokio::spawn(async move {
            builder_api.run().await;
        }));

        join_all(tasks).await;
    }
}
