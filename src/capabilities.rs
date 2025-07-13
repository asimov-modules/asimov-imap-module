// This is free and unencumbered software released into the public domain.

/// See: https://www.iana.org/assignments/imap-capabilities/imap-capabilities.xhtml
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ImapCapabilities {
    /// See: https://datatracker.ietf.org/doc/html/rfc5161
    pub enable: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc4731
    pub esearch: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc2971
    pub id: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc2177
    pub idle: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc5258
    pub list_extended: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc5819
    pub list_status: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc5256
    pub sort: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc9586
    pub uidonly: bool,
    /// See: https://datatracker.ietf.org/doc/html/rfc9755
    pub utf8_accept: bool,
    /// See: https://developers.google.com/workspace/gmail/imap/imap-extensions
    pub x_gm_ext_1: bool,
}

impl ImapCapabilities {
    pub fn is_gmail(&self) -> bool {
        self.x_gm_ext_1
    }
}

impl From<imap::types::Capabilities> for ImapCapabilities {
    fn from(input: imap::types::Capabilities) -> Self {
        let mut output = ImapCapabilities::default();
        for capability in input.iter() {
            use imap_proto::Capability::*;
            match capability {
                Imap4rev1 => {},
                Auth(_) => {},
                Atom(s) => match s.as_ref() {
                    "ENABLE" => output.enable = true,
                    "ESEARCH" => output.esearch = true,
                    "ID" => output.id = true,
                    "IDLE" => output.idle = true,
                    "LIST-EXTENDED" => output.list_extended = true,
                    "LIST-STATUS" => output.list_status = true,
                    "SORT" => output.sort = true,
                    "UIDONLY" => output.uidonly = true,
                    "UTF8=ACCEPT" | "UTF8=ONLY" => output.utf8_accept = true,
                    "X-GM-EXT-1" => output.x_gm_ext_1 = true,
                    _ => {},
                },
            }
        }
        output
    }
}
