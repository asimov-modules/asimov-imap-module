// This is free and unencumbered software released into the public domain.

use dogma::{Uri, UriScheme};
use secrecy::SecretString;

/// See: https://datatracker.ietf.org/doc/html/rfc5092
#[derive(Clone, Debug)]
pub struct ImapUrl {
    pub scheme: UriScheme,
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<SecretString>,
    pub mailbox: String,
}

impl ImapUrl {
    pub fn is_tls(&self) -> bool {
        self.scheme == UriScheme::Imaps
    }
}

impl From<&Uri<'_>> for ImapUrl {
    fn from(url: &Uri) -> Self {
        use percent_encoding::percent_decode_str;
        let url_authority = url.authority().unwrap();
        let is_tls = url.scheme() == UriScheme::Imaps;
        let mailbox = match url.path().strip_prefix('/') {
            None | Some("") => "INBOX".to_string(),
            Some(name) => percent_decode_str(name).decode_utf8_lossy().into_owned(),
        };

        Self {
            scheme: url.scheme(),
            host: url_authority.host_str().to_lowercase(),
            port: url_authority
                .port()
                .unwrap_or_else(|| if is_tls { 993 } else { 143 }),
            user: url_authority.username().map(ToString::to_string),
            password: url_authority.password().map(|password| password.into()),
            mailbox,
        }
    }
}
