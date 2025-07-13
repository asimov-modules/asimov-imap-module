// This is free and unencumbered software released into the public domain.

use super::{
    ImapCapabilities, ImapError, ImapIterator, ImapLocalCursor, ImapMessage, ImapOrderBy, ImapUrl,
};
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
        let fetch_query = "(UID FLAGS ENVELOPE)";
        Ok(match (self.capabilities.sort, &order_by) {
            (_, ImapOrderBy::None) => {
                ImapIterator::new(self.session.fetch("1:*", fetch_query)?, None)
            },
            (true, _) => {
                use imap::extensions::sort::SortCharset;
                let mut uid_set =
                    self.session
                        .uid_sort(&[order_by.into()], SortCharset::Utf8, "ALL")?;

                if let Some(limit) = limit {
                    uid_set.truncate(limit);
                }

                let mut cursor = String::new();
                for (i, uid) in uid_set.iter().enumerate() {
                    use core::fmt::Write;
                    if i > 0 {
                        cursor.push(',');
                    }
                    write!(&mut cursor, "{}", uid).unwrap();
                }

                let fetches = self.session.uid_fetch(cursor.to_string(), fetch_query)?;
                ImapIterator::new(fetches, None)
            },
            (false, ImapOrderBy::Timestamp) => {
                let cursor = ImapLocalCursor::<i64>::by_timestamp(&mut self.session)?.limit(limit);
                let fetches = self.session.uid_fetch(cursor.to_string(), fetch_query)?;
                ImapIterator::new(fetches, Some(cursor.to_vec()))
            },
            (false, ImapOrderBy::Date) => {
                let cursor = ImapLocalCursor::<i64>::by_date(&mut self.session)?.limit(limit);
                let fetches = self.session.uid_fetch(cursor.to_string(), fetch_query)?;
                ImapIterator::new(fetches, Some(cursor.to_vec()))
            },
            (false, ImapOrderBy::From) => {
                let cursor = ImapLocalCursor::<String>::by_from(&mut self.session)?.limit(limit);
                let fetches = self.session.uid_fetch(cursor.to_string(), fetch_query)?;
                ImapIterator::new(fetches, Some(cursor.to_vec()))
            },
            (false, ImapOrderBy::To) => {
                let cursor = ImapLocalCursor::<String>::by_to(&mut self.session)?.limit(limit);
                let fetches = self.session.uid_fetch(cursor.to_string(), fetch_query)?;
                ImapIterator::new(fetches, Some(cursor.to_vec()))
            },
            (false, ImapOrderBy::Cc) => {
                let cursor = ImapLocalCursor::<String>::by_cc(&mut self.session)?.limit(limit);
                let fetches = self.session.uid_fetch(cursor.to_string(), fetch_query)?;
                ImapIterator::new(fetches, Some(cursor.to_vec()))
            },
            (false, ImapOrderBy::Size) => {
                let cursor = ImapLocalCursor::<usize>::by_size(&mut self.session)?.limit(limit);
                let fetches = self.session.uid_fetch(cursor.to_string(), fetch_query)?;
                ImapIterator::new(fetches, Some(cursor.to_vec()))
            },
        })
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
