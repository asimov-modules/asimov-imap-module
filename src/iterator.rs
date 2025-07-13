// This is free and unencumbered software released into the public domain.

use super::ImapMessage;
use core::error::Error;
use imap::types::Fetches;

pub struct ImapIterator {
    index: usize,
    ordering: Option<Vec<u32>>,
    fetches: Fetches,
}

impl ImapIterator {
    pub fn new(fetches: Fetches, uids: Option<Vec<u32>>) -> Self {
        let Some(uids) = &uids else {
            return Self {
                index: 0,
                ordering: None,
                fetches,
            };
        };
        assert_eq!(fetches.len(), uids.len());
        let count = fetches.len();
        let mut ordering: Vec<u32> = Vec::with_capacity(count);
        for &uid in uids.iter() {
            let position = fetches
                .iter()
                .position(|fetch| fetch.uid == Some(uid))
                .expect("ImapIterator should be correctly constructed");
            ordering.push(position as _);
        }
        Self {
            index: 0,
            ordering: Some(ordering),
            fetches,
        }
    }
}

impl Iterator for ImapIterator {
    type Item = Result<ImapMessage, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::{Error, ErrorKind};
        if self.index < self.fetches.len() {
            let position = self
                .ordering
                .as_ref()
                .map(|ordering| ordering[self.index])
                .unwrap_or(self.index as u32);
            self.index += 1;
            let fetch = self.fetches.get(position as _)?;
            let result = fetch
                .try_into()
                .map_err(|err| Box::new(Error::new(ErrorKind::Other, err)) as _);
            Some(result)
        } else {
            None
        }
    }
}
