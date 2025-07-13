// This is free and unencumbered software released into the public domain.

use super::{ImapCapabilities, ImapError, ImapIterator, ImapMessage, ImapUrl};
use asimov_module::tracing;
use core::error::Error;
use imap::{ClientBuilder, ConnectionMode, ImapConnection, Session, TlsKind, types::Mailbox};
use know::datatypes::EmailMessageId;
use secrecy::ExposeSecret;

#[allow(unused)]
pub struct ImapReader {
    session: Session<Box<dyn ImapConnection + 'static>>,
    capabilities: ImapCapabilities,
    mailbox: Mailbox,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum ImapOrderBy {
    /// The default server order
    #[default]
    None,
    /// Sent date and time
    Date,
    /// The first From address
    From,
    /// The first To address
    To,
    /// The first Cc address
    Cc,
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

        let capabilities = session.capabilities()?.into();
        tracing::trace!("{:?}", capabilities);

        let mailbox = session.select(&url.mailbox)?;
        Ok(Self {
            session,
            capabilities,
            mailbox,
        })
    }

    pub fn close(&mut self) -> imap::Result<()> {
        self.session.logout()
    }

    pub fn iter(
        &mut self,
        order_by: ImapOrderBy,
        limit: Option<usize>,
    ) -> imap::Result<impl Iterator<Item = Result<ImapMessage, Box<dyn Error>>>> {
        let fetches = match (order_by, self.capabilities.sort) {
            (ImapOrderBy::None, _) => self.session.fetch("1:*", "(UID FLAGS ENVELOPE)")?,
            (ImapOrderBy::Date, true) => {
                use imap::extensions::sort::{SortCharset, SortCriterion};
                let mut uid_set = self.session.uid_sort(
                    &[SortCriterion::Reverse(&SortCriterion::Arrival)],
                    SortCharset::Utf8,
                    "ALL",
                )?;

                if let Some(limit) = limit {
                    uid_set.truncate(limit);
                }

                let mut uid_buffer = String::new();
                for (i, uid) in uid_set.iter().enumerate() {
                    use core::fmt::Write;
                    if i > 0 {
                        uid_buffer.push(',');
                    }
                    write!(&mut uid_buffer, "{}", uid).unwrap();
                }

                self.session.uid_fetch(uid_buffer, "(UID FLAGS ENVELOPE)")?
            },
            (ImapOrderBy::Date, false) => {
                let mut timestamp_to_uid: Vec<(i64, u32)> = Vec::new();
                let fetches = self.session.fetch("1:*", "(UID INTERNALDATE)")?;
                for fetch in fetches.iter() {
                    let uid = fetch.uid.unwrap();
                    let timestamp = fetch.internal_date().unwrap().timestamp_millis();
                    timestamp_to_uid.push((timestamp, uid));
                }
                timestamp_to_uid.sort();
                timestamp_to_uid.reverse();

                if let Some(limit) = limit {
                    timestamp_to_uid.truncate(limit);
                }

                let mut uid_buffer = String::new();
                for (i, (_, uid)) in timestamp_to_uid.iter().enumerate() {
                    use core::fmt::Write;
                    if i > 0 {
                        uid_buffer.push(',');
                    }
                    write!(&mut uid_buffer, "{}", uid).unwrap();
                }

                self.session.uid_fetch(uid_buffer, "(UID FLAGS ENVELOPE)")?
            },
            (ImapOrderBy::From, _) => todo!(), // TODO
            (ImapOrderBy::To, _) => todo!(),   // TODO
            (ImapOrderBy::Cc, _) => todo!(),   // TODO
        };
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
