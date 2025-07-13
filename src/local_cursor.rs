// This is free and unencumbered software released into the public domain.

use imap::{ImapConnection, Session};
use know::datatypes::EmailAddress;

#[derive(Clone, Debug, Default)]
pub struct ImapLocalCursor<T> {
    pub(crate) val_to_uid: Vec<(T, u32)>,
}

impl<T> ImapLocalCursor<T> {
    pub fn by_timestamp(
        session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<i64>> {
        let mut val_to_uid = vec![];
        let fetches = session.fetch("1:*", "(UID INTERNALDATE)")?;
        for fetch in fetches.iter() {
            let uid = fetch.uid.unwrap();
            let timestamp = fetch.internal_date().unwrap().timestamp_millis();
            val_to_uid.push((timestamp, uid));
        }
        val_to_uid.sort();
        val_to_uid.reverse();
        Ok(ImapLocalCursor { val_to_uid })
    }

    pub fn by_date(
        session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<i64>> {
        let mut val_to_uid = vec![];
        let fetches = session.fetch("1:*", "(UID ENVELOPE)")?; // TODO: optimize
        for fetch in fetches.iter() {
            let uid = fetch.uid.unwrap();
            let Some(date_header) = fetch.envelope().unwrap().date.as_ref() else {
                continue; // skip messages without a `Date:` header
            };
            let Ok(date) = core::str::from_utf8(date_header) else {
                continue; // skip messages with invalid UTF-8 in the header
            };
            let Ok(datetime) = jiff::fmt::rfc2822::parse(date) else {
                continue; // skip messages with an invalid `Date:` header
            };
            let timestamp = datetime.timestamp().as_millisecond();
            val_to_uid.push((timestamp, uid));
        }
        val_to_uid.sort();
        val_to_uid.reverse();
        Ok(ImapLocalCursor { val_to_uid })
    }

    pub fn by_from(
        session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<String>> {
        let mut val_to_uid = vec![];
        let fetches = session.fetch("1:*", "(UID ENVELOPE)")?; // TODO: optimize
        for fetch in fetches.iter() {
            let uid = fetch.uid.unwrap();
            let Some(from_header) = fetch.envelope().unwrap().from.as_ref() else {
                continue; // skip messages without a `From:` header
            };
            let Some(from) = from_header.iter().next() else {
                continue; // skip messages without a `From:` address
            };
            let Ok(email) = EmailAddress::try_from(from) else {
                continue; // skip messages with invalid email addresses
            };
            val_to_uid.push((email.into(), uid));
        }
        val_to_uid.sort();
        Ok(ImapLocalCursor { val_to_uid })
    }

    pub fn by_to(
        session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<String>> {
        let mut val_to_uid = vec![];
        let fetches = session.fetch("1:*", "(UID ENVELOPE)")?; // TODO: optimize
        for fetch in fetches.iter() {
            let uid = fetch.uid.unwrap();
            let Some(from_header) = fetch.envelope().unwrap().to.as_ref() else {
                continue; // skip messages without a `To:` header
            };
            let Some(from) = from_header.iter().next() else {
                continue; // skip messages without a `To:` address
            };
            let Ok(email) = EmailAddress::try_from(from) else {
                continue; // skip messages with invalid email addresses
            };
            val_to_uid.push((email.into(), uid));
        }
        val_to_uid.sort();
        Ok(ImapLocalCursor { val_to_uid })
    }

    pub fn by_cc(
        session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<String>> {
        let mut val_to_uid = vec![];
        let fetches = session.fetch("1:*", "(UID ENVELOPE)")?; // TODO: optimize
        for fetch in fetches.iter() {
            let uid = fetch.uid.unwrap();
            let Some(from_header) = fetch.envelope().unwrap().cc.as_ref() else {
                continue; // skip messages without a `Cc:` header
            };
            let Some(from) = from_header.iter().next() else {
                continue; // skip messages without a `Cc:` address
            };
            let Ok(email) = EmailAddress::try_from(from) else {
                continue; // skip messages with invalid email addresses
            };
            val_to_uid.push((email.into(), uid));
        }
        val_to_uid.sort();
        Ok(ImapLocalCursor { val_to_uid })
    }

    pub fn by_size(
        session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<u32>> {
        let mut val_to_uid = vec![];
        let fetches = session.fetch("1:*", "(UID RFC822.SIZE)")?;
        for fetch in fetches.iter() {
            let uid = fetch.uid.unwrap();
            let size = fetch.size.unwrap();
            val_to_uid.push((size, uid));
        }
        val_to_uid.sort();
        val_to_uid.reverse();
        Ok(ImapLocalCursor { val_to_uid })
    }

    pub fn limit(mut self, count: Option<usize>) -> Self {
        if let Some(count) = count {
            self.val_to_uid.truncate(count);
        }
        self
    }

    pub fn to_vec(&self) -> Vec<u32> {
        self.val_to_uid.iter().map(|(_, uid)| *uid).collect()
    }
}

impl<T> ToString for ImapLocalCursor<T> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        for (i, (_, uid)) in self.val_to_uid.iter().enumerate() {
            use core::fmt::Write;
            if i > 0 {
                output.push(',');
            }
            write!(&mut output, "{}", uid).unwrap();
        }
        output
    }
}
