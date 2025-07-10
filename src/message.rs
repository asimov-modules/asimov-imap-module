// This is free and unencumbered software released into the public domain.

use super::ImapError;
use imap::types::Fetch;
use know::classes::EmailMessage;
use mail_parser::MessageParser;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImapMessage {
    pub pos: usize,
    pub uid: Option<u32>,
    pub size: Option<u32>,
    pub headers: EmailMessage,
    pub body: Option<String>,
}

impl TryFrom<&Fetch<'_>> for ImapMessage {
    type Error = ImapError;

    fn try_from(input: &Fetch) -> Result<Self, Self::Error> {
        Ok(match input.body() {
            Some(bytes) => {
                let message = MessageParser::default()
                    .parse(bytes)
                    .ok_or(ImapError::InvalidMessage)?;
                Self {
                    pos: input.message as _,
                    uid: input.uid,
                    size: input.size,
                    headers: (&message)
                        .try_into()
                        .map_err(|_| ImapError::InvalidHeaders)?,
                    body: message.body_text(0).map(|s| s.into_owned()),
                }
            },
            None => Self {
                pos: input.message as _,
                uid: input.uid,
                size: input.size,
                headers: input
                    .envelope()
                    .unwrap()
                    .try_into()
                    .map_err(|_| ImapError::InvalidHeaders)?,
                body: None,
            },
        })
    }
}
