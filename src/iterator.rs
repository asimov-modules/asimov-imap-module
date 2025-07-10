// This is free and unencumbered software released into the public domain.

use super::{ImapError, ImapMessage};
use core::error::Error;
use imap::types::Fetches;

pub struct ImapIterator {
    inner: Fetches,
    index: usize,
}

impl ImapIterator {
    pub fn new(inner: Fetches) -> Self {
        Self { inner, index: 0 }
    }
}

impl Iterator for ImapIterator {
    type Item = Result<ImapMessage, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::{Error, ErrorKind};
        if self.index < self.inner.len() {
            let fetch = self.inner.get(self.index)?;
            self.index += 1;
            let result = fetch
                .try_into()
                .map_err(|err| Box::new(Error::new(ErrorKind::Other, err)) as _);
            Some(result)
        } else {
            None
        }
    }
}
