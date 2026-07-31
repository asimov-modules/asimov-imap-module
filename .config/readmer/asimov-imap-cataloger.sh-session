$ asimov-imap-cataloger
asimov-imap-cataloger

Usage: asimov-imap-cataloger [OPTIONS] <IMAP-MAILBOX-URL>

Arguments:
  <IMAP-MAILBOX-URL>  An `imaps://user@host:port/mailbox` (or `imap://...`) URL to the IMAP mailbox to catalog

Options:
  -d, --debug                Enable debugging output
      --license              Show license information
  -v, --verbose...           Enable verbose output (may be repeated for more verbosity)
  -V, --version              Print version information
  -b, --order-by <PROPERTY>  Order messages by a property [default: none] [possible values: none, timestamp, date, from, to, cc, size]
  -n, --limit <COUNT>        Limit the number of messages to catalog
  -o, --output <FORMAT>      Set the output format [default: cli] [possible values: cli, json, jsonld, jsonl, tldr]
  -h, --help                 Print help (see more with '--help')
