// This is free and unencumbered software released into the public domain.

use imap::types::Fetch;
use imap_proto::types::Envelope;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImapMessage {
    pub id: String,
}

impl TryFrom<&Fetch<'_>> for ImapMessage {
    type Error = &'static str;

    fn try_from(input: &Fetch) -> Result<Self, Self::Error> {
        input.envelope().unwrap().try_into()
    }
}

impl TryFrom<&Envelope<'_>> for ImapMessage {
    type Error = &'static str;

    fn try_from(input: &Envelope) -> Result<Self, Self::Error> {
        let id = input
            .message_id
            .as_ref()
            .ok_or("message must have a Message-ID")?;
        let id = String::from_utf8_lossy(id).to_string();
        Ok(ImapMessage { id })
    }
}
