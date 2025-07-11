// This is free and unencumbered software released into the public domain.

use crate::ImapUrl;
use clientele::envs as getenv;
use core::error::Error;
use netrc::Netrc;

pub struct ImapConfiguration {
    netrc: Netrc,
}

impl ImapConfiguration {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            netrc: Netrc::new()?,
        })
    }

    pub fn resolve_url(&self, mut url: ImapUrl) -> Result<ImapUrl, Box<dyn Error>> {
        if url.password.is_none() {
            let env_user = getenv::var("ASIMOV_IMAP_USER");
            let env_password = getenv::var("ASIMOV_IMAP_PASSWORD");
            if let (Some(user), Some(password)) = (env_user, env_password) {
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
        }
        Ok(url)
    }

    pub fn get_creds(&self, host: &String, port: Option<u16>) -> Option<(String, String)> {
        let entry_key = if let Some(port) = port {
            format!("{}:{}", host, port)
        } else {
            host.clone()
        };
        let entry = self.netrc.hosts.get(&entry_key)?;
        Some((entry.login.clone(), entry.password.clone()))
    }
}
