// This is free and unencumbered software released into the public domain.

use imap::{ImapConnection, Session};

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
        _session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<i64>> {
        todo!()
    }

    pub fn by_from(
        _session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<String>> {
        todo!()
    }

    pub fn by_to(
        _session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<String>> {
        todo!()
    }

    pub fn by_cc(
        _session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<String>> {
        todo!()
    }

    pub fn by_size(
        _session: &mut Session<Box<dyn ImapConnection + 'static>>,
    ) -> imap::error::Result<ImapLocalCursor<usize>> {
        todo!()
    }

    pub fn limit(mut self, count: Option<usize>) -> Self {
        if let Some(count) = count {
            self.val_to_uid.truncate(count);
        }
        self
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
