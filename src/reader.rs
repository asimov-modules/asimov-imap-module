// This is free and unencumbered software released into the public domain.

use crate::ImapUrl;

use super::{ImapIterator, ImapMessage};
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

    pub fn fetch(&mut self, mid: &EmailMessageId) -> Result<Option<ImapMessage>, Box<dyn Error>> {
        for entry in self.iter()? {
            let message = entry?;
            if let Some(message_id) = message.message.id.as_ref() {
                if message_id == mid {
                    return Ok(Some(message));
                }
            }
        }
        Ok(None)
    }
}
