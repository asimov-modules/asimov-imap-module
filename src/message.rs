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
                
                let headers: EmailMessage = (&message)
                    .try_into()
                    .map_err(|_| ImapError::InvalidHeaders)?;
                
                Self {
                    pos: input.message as _,
                    uid: input.uid,
                    size: input.size,
                    headers: headers.clone(),
                    body: headers.body,
                }
            },
            None => {
                let envelope = input.envelope().unwrap();
                let mut headers: EmailMessage =
                    envelope.try_into().map_err(|_| ImapError::InvalidHeaders)?;
                if let Some(subject_bytes) = envelope.subject.as_ref() {
                    headers.subject = rfc2047_decoder::decode(subject_bytes).ok();
                }
                Self {
                    pos: input.message as _,
                    uid: input.uid,
                    size: input.size,
                    headers,
                    body: None,
                }
            },
        })
    }
}
