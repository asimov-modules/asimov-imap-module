// This is free and unencumbered software released into the public domain.

use crate::ImapUrl;
use asimov_module::ModuleManifest;
use core::error::Error;
use netrc::Netrc;

pub struct ImapConfiguration {
    manifest: Option<ModuleManifest>,
    netrc: Option<Netrc>,
}

impl ImapConfiguration {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            manifest: ModuleManifest::read_manifest("imap").ok(),
            netrc: Netrc::new().ok(),
        })
    }

    pub fn resolve_url(&self, mut url: ImapUrl) -> Result<ImapUrl, Box<dyn Error>> {
        if url.password.is_some() {
            return Ok(url);
        }

        let configured_user = self
            .manifest
            .as_ref()
            .and_then(|manifest| manifest.variable("user", None).ok());
        let configured_password = self
            .manifest
            .as_ref()
            .and_then(|manifest| manifest.variable("password", None).ok());
        if let (Some(user), Some(password)) = (configured_user, configured_password) {
            if !user.is_empty() && !password.is_empty() {
                (url.user, url.password) = (Some(user), Some(password.into()));
                return Ok(url);
            }
        }

        if let Some((user, password)) = self.get_creds(&url.host, Some(url.port)) {
            if !user.is_empty() && !password.is_empty() {
                (url.user, url.password) = (Some(user), Some(password.into()));
                return Ok(url);
            }
        }

        if let Some((user, password)) = self.get_creds(&url.host, None) {
            if !user.is_empty() && !password.is_empty() {
                (url.user, url.password) = (Some(user), Some(password.into()));
                return Ok(url);
            }
        }

        // Default to anonymous login if no credentials were configured,
        // enabling e.g. use of hosts such as `imaps://imap.ietf.org`:
        (url.user, url.password) = (
            Some("anonymous".into()),
            Some("support@asimov.systems".into()),
        );

        Ok(url)
    }

    pub fn get_creds(&self, host: &String, port: Option<u16>) -> Option<(String, String)> {
        let Some(netrc) = self.netrc.as_ref() else {
            return None;
        };
        let entry_key = if let Some(port) = port {
            format!("{}:{}", host, port)
        } else {
            host.clone()
        };
        let entry = netrc.hosts.get(&entry_key)?;
        Some((entry.login.clone(), entry.password.clone()))
    }
}
