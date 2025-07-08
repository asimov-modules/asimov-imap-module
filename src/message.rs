// This is free and unencumbered software released into the public domain.

use imap::types::Fetch;
use know::classes::EmailMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImapMessage {
    pub pos: usize,
    pub uid: Option<u32>,
    pub size: Option<u32>,
    pub message: EmailMessage,
}

impl TryFrom<&Fetch<'_>> for ImapMessage {
    type Error = &'static str;

    fn try_from(input: &Fetch) -> Result<Self, Self::Error> {
        Ok(Self {
            pos: input.message as _,
            uid: input.uid,
            size: input.size,
            message: input
                .envelope()
                .unwrap()
                .try_into()
                .map_err(|_| "failed to parse message")?,
        })
    }
}
