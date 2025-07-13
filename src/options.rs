// This is free and unencumbered software released into the public domain.

use imap::extensions::sort::SortCriterion;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum ImapOrderBy {
    /// The default server order
    #[default]
    None,

    /// The server's received date and time
    Timestamp,

    /// The sender's sent date and time
    Date,

    /// The first From address
    From,

    /// The first To address
    To,

    /// The first Cc address
    Cc,

    /// The size of the message
    Size,
}

impl Into<SortCriterion<'_>> for ImapOrderBy {
    fn into(self) -> SortCriterion<'static> {
        use ImapOrderBy::*;
        match self {
            None => SortCriterion::Arrival,
            Timestamp => SortCriterion::Reverse(&SortCriterion::Arrival),
            Date => SortCriterion::Reverse(&SortCriterion::Date),
            From => SortCriterion::From,
            To => SortCriterion::To,
            Cc => SortCriterion::Cc,
            Size => SortCriterion::Size,
        }
    }
}
