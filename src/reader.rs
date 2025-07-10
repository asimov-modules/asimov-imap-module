// This is free and unencumbered software released into the public domain.

use super::{ImapError, ImapIterator, ImapMessage, ImapUrl};
use core::error::Error;
use imap::{ClientBuilder, ConnectionMode, ImapConnection, Session, TlsKind, types::Mailbox};
use know::datatypes::EmailMessageId;
use secrecy::ExposeSecret;

pub struct ImapReader {
    session: Session<Box<dyn ImapConnection + 'static>>,
    #[allow(unused)]
    mailbox: Mailbox,
}

impl ImapReader {
    pub fn open(url: &ImapUrl) -> imap::Result<Self> {
        let client = ClientBuilder::new(&url.host, url.port)
            .mode(ConnectionMode::Tls)
            .tls_kind(TlsKind::Rust)
            .danger_skip_tls_verify(true)
            .connect()?;

        let mut session = client
            .login(
                url.user.clone().unwrap_or_else(|| "anonymous".to_string()),
                url.password
                    .as_ref()
                    .map(|password| password.expose_secret())
                    .unwrap_or_default(),
            )
            .map_err(|e| e.0)?;

        let mailbox = session.select(&url.mailbox)?;
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

    pub fn fetch(&mut self, message_id: &EmailMessageId) -> Result<Option<ImapMessage>, ImapError> {
        let message_uid = self
            .session
            .uid_search(format!("HEADER Message-ID <{}>", message_id.as_str()))?
            .into_iter()
            .next();
        let Some(message_uid) = message_uid else {
            return Ok(None);
        };
        let fetch_results = self
            .session
            .uid_fetch(message_uid.to_string(), "(BODY[])")?;
        let Some(fetch_result) = fetch_results.get(0) else {
            return Ok(None);
        };
        Ok(Some(fetch_result.try_into()?))
    }
}
