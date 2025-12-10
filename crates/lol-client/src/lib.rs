use std::sync::Arc;

use native_tls::TlsConnector;
use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;
use ureq::{Agent, AgentBuilder};

use ugg_types::client_champ_select::ChampSelectSession;
use ugg_types::client_runepage::{NewRunePage, RunePage, RunePages};
use ugg_types::client_summoner::ClientSummoner;

mod lcc;
use lcc::{LeagueClientConnector, RiotLockFile};

#[derive(Error, Debug)]
pub enum LOLClientError {
    #[error("Unable to create TLS connector")]
    TlsConnectorError(#[from] native_tls::Error),
    #[error("Unable to read lockfile")]
    LockfileReadError(#[from] lcc::LeagueConnectorError),
    #[error("Linux is not supported")]
    LinuxNotSupported,
}

pub struct LOLClientAPI {
    agent: Agent,
    lockfile: RiotLockFile,
}

impl LOLClientAPI {
    pub fn new() -> Result<LOLClientAPI, LOLClientError> {
        if cfg!(target_os = "linux") {
            return Err(LOLClientError::LinuxNotSupported);
        }
        Ok(LOLClientAPI {
            agent: AgentBuilder::new()
                .tls_connector(Arc::new(
                    TlsConnector::builder()
                        .danger_accept_invalid_certs(true)
                        .build()?,
                ))
                .build(),
            lockfile: LeagueClientConnector::parse_lockfile()?,
        })
    }

    fn make_url(&self, endpoint: &str) -> String {
        format!("https://127.0.0.1:{}{}", self.lockfile.port, endpoint)
    }

    fn get_data<T: DeserializeOwned>(&self, endpoint: &str) -> Option<T> {
        match self
            .agent
            .get(&self.make_url(endpoint))
            .set(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .call()
        {
            Ok(response) => {
                if response.status() == 200 {
                    response.into_json().ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn delete_data(&self, endpoint: &str) {
        let _ = self
            .agent
            .delete(&self.make_url(endpoint))
            .set(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .call();
    }

    fn post_data<T: Serialize>(&self, endpoint: &str, data: &T) {
        let _ = self
            .agent
            .post(&self.make_url(endpoint))
            .set(
                "Authorization",
                &format!("Basic {}", self.lockfile.b64_auth),
            )
            .send_json(data);
    }

    #[must_use]
    pub fn get_summoner_info(&self) -> Option<ClientSummoner> {
        self.get_data::<ClientSummoner>("/lol-summoner/v1/current-summoner")
    }

    #[must_use]
    pub fn get_current_rune_page(&self) -> Option<RunePage> {
        match self.get_data::<RunePages>("/lol-perks/v1/pages") {
            Some(data) => {
                for page in &data {
                    if page.name.starts_with("uggo:") && page.is_deletable {
                        return Some(page.clone());
                    }
                }
                for page in &data {
                    if page.current && page.is_deletable {
                        return Some(page.clone());
                    }
                }
                None
            }
            None => None,
        }
    }

    pub fn update_rune_page(&self, old_page_id: i64, rune_page: &NewRunePage) {
        self.delete_data(&format!("/lol-perks/v1/pages/{}", old_page_id));
        self.post_data("/lol-perks/v1/pages", rune_page);
    }

    pub fn get_champ_select_session(&self) -> Option<ChampSelectSession> {
        self.get_data::<ChampSelectSession>("/lol-champ-select/v1/session")
    }
}
