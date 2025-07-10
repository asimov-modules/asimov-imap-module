// This is free and unencumbered software released into the public domain.

use core::fmt;

#[derive(Clone, Debug)]
pub enum ImapError {
    InvalidEntry,
    InvalidMessage,
    InvalidHeaders,
    UnexpectedResponse,
    UnexpectedError,
}

impl core::error::Error for ImapError {}

impl fmt::Display for ImapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImapError::*;
        match self {
            InvalidEntry => write!(f, "invalid IMAP entry"),
            InvalidMessage => write!(f, "invalid IMAP message"),
            InvalidHeaders => write!(f, "invalid IMAP headers"),
            UnexpectedResponse => write!(f, "unexpected IMAP response"),
            UnexpectedError => write!(f, "unexpected IMAP error"),
        }
    }
}

impl From<imap::Error> for ImapError {
    fn from(input: imap::Error) -> Self {
        use imap::Error::*;
        match input {
            Io(_) => Self::UnexpectedError,
            #[cfg(feature = "tls")]
            RustlsHandshake(_) => Self::UnexpectedError,
            Bad(_) => Self::UnexpectedError,
            No(_) => Self::UnexpectedError,
            Bye(_) => Self::UnexpectedError,
            ConnectionLost => Self::UnexpectedError,
            Parse(_) => Self::UnexpectedError,
            Validate(_) => Self::UnexpectedError,
            Append => unreachable!(),
            Unexpected(_) => Self::UnexpectedResponse,
            MissingStatusResponse => Self::UnexpectedError,
            TagMismatch(_) => Self::UnexpectedError,
            StartTlsNotAvailable => Self::UnexpectedError,
            TlsNotConfigured => Self::UnexpectedError,
            _ => Self::UnexpectedError,
        }
    }
}
