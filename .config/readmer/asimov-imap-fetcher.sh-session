$ asimov-imap-fetcher --help
asimov-imap-fetcher

Usage: asimov-imap-fetcher [OPTIONS] <IMAP-MESSAGE-URL>

Arguments:
  <IMAP-MESSAGE-URL>  An `imaps://user@host:port/mailbox#mid` (or `imap://...`) URL to the message to fetch

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  Set the output format [default: cli] [possible values: cli, json, jsonld, mime, tldr]
  -h, --help             Print help
