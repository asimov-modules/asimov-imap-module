// This is free and unencumbered software released into the public domain.

use crate::ImapOrderBy;
use imap::{ImapConnection, Session};

#[derive(Clone, Debug, Default)]
pub struct ImapRemoteCursor {
    pub(crate) uids: Vec<u32>,
}

impl ImapRemoteCursor {
    pub fn by(
        session: &mut Session<Box<dyn ImapConnection + 'static>>,
        order_by: ImapOrderBy,
    ) -> imap::error::Result<ImapRemoteCursor> {
        use imap::extensions::sort::SortCharset;
        let uids = session.uid_sort(&[order_by.into()], SortCharset::Utf8, "ALL")?;
        Ok(ImapRemoteCursor { uids })
    }

    pub fn limit(mut self, count: Option<usize>) -> Self {
        if let Some(count) = count {
            self.uids.truncate(count);
        }
        self
    }

    pub fn to_vec(&self) -> Vec<u32> {
        self.uids.clone()
    }
}

impl ToString for ImapRemoteCursor {
    fn to_string(&self) -> String {
        let mut output = String::new();
        for (i, uid) in self.uids.iter().enumerate() {
            use core::fmt::Write;
            if i > 0 {
                output.push(',');
            }
            write!(&mut output, "{}", uid).unwrap();
        }
        output
    }
}
