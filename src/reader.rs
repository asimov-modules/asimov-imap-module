// This is free and unencumbered software released into the public domain.

use super::{ImapIterator, ImapMessage};
use core::error::Error;
use dogma::UriScheme;
use imap::{ClientBuilder, ConnectionMode, ImapConnection, Session, TlsKind, types::Mailbox};

pub struct ImapReader {
    session: Session<Box<dyn ImapConnection + 'static>>,
    #[allow(unused)]
    mailbox: Mailbox,
}

impl ImapReader {
    pub fn open(uri: dogma::Uri) -> imap::Result<Self> {
        let is_tls = match uri.scheme() {
            UriScheme::Imap => false,
            UriScheme::Imaps => true,
            _ => return Err(imap::Error::TlsNotConfigured), // TODO
        };
        let uri_authority = uri.authority().expect("URI authority is required");
        let username = uri_authority.username().unwrap_or_default();
        let password = uri_authority.password().unwrap_or_default();
        let host = uri_authority.host_str();
        let port = uri_authority
            .port()
            .unwrap_or_else(|| if is_tls { 993 } else { 143 });
        let first_segment = uri
            .path_segments()
            .map(|mut segments| segments.next().unwrap_or_default());

        let client = ClientBuilder::new(host, port)
            .mode(ConnectionMode::Tls)
            .tls_kind(TlsKind::Rust)
            .danger_skip_tls_verify(true)
            .connect()?;

        let mut session = client.login(username, password).map_err(|e| e.0)?;
        let mailbox = session.select(match first_segment {
            None | Some("") => "INBOX",
            Some(name) => name,
        })?;
        Ok(Self { session, mailbox })
    }

    pub fn close(&mut self) -> imap::Result<()> {
        self.session.logout()
    }

    pub fn iter(
        &mut self,
    ) -> imap::Result<impl Iterator<Item = Result<ImapMessage, Box<dyn Error>>>> {
        let fetches = self.session.fetch("1:*", "(UID FLAGS ENVELOPE)")?;
        Ok(ImapIterator::new(fetches))
    }
}
