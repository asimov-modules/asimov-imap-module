# ASIMOV IMAP Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-imap-module)](https://crates.io/crates/asimov-imap-module)
[![Documentation](https://docs.rs/asimov-imap-module/badge.svg)](https://docs.rs/asimov-imap-module)

[ASIMOV] module for [IMAP] email import.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install imap -v
```

### Installation from Source Code

```bash
cargo install asimov-imap-module
```

## 👉 Examples

```bash
asimov-imap-cataloger imaps://user@host:port/mailbox
```

```bash
asimov-imap-fetcher imaps://user@host:port/mailbox#mid
```

## ⚙ Configuration

### Authentication Credentials

Typically, authentication credentials are required to access an IMAP mailbox.
These can be supplied in three different ways, listed below in order of
precedence:

#### 1. Configuring credentials in the `imaps:` URL

Authentication credentials can be supplied inline in the URL, as follows:

```bash
asimov-imap-cataloger imaps://myuser:mypassword@host:port/mailbox
```

#### 2. Configuring credentials in environment variables

Authentication credentials can also be supplied via environment variables, as
follows:

```bash
export ASIMOV_IMAP_USER=myuser
export ASIMOV_IMAP_PASSWORD=mypassword

asimov-imap-cataloger imaps://host:port/mailbox
```

#### 3. Configuring credentials in the `~/.netrc` file

Authentication credentials can also be supplied via the `~/.netrc` (aka
`$HOME/.netrc`) file, as follows:

```
machine host
login myuser
password mypassword
```

```bash
asimov-imap-cataloger imaps://host:port/mailbox
```

## 📚 Reference

### `asimov-imap-cataloger`

```
asimov-imap-cataloger

Usage: asimov-imap-cataloger [OPTIONS] <IMAP-MAILBOX-URL>

Arguments:
  <IMAP-MAILBOX-URL>  An `imaps://user@host:port/mailbox` (or `imap://...`) URL to the IMAP mailbox to catalog

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -n, --limit <COUNT>    The maximum number of messages to catalog
  -o, --output <FORMAT>  The output format
  -h, --help             Print help
```

### `asimov-imap-fetcher`

```
asimov-imap-fetcher

Usage: asimov-imap-fetcher [OPTIONS] <IMAP-MESSAGE-URL>

Arguments:
  <IMAP-MESSAGE-URL>  An `imaps://user@host:port/mailbox#mid` (or `imap://...`) URL to the message to fetch

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  The output format
  -h, --help             Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-imap-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-imap-module&text=asimov-imap-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-imap-module&title=asimov-imap-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-imap-module&t=asimov-imap-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-imap-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-imap-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[IMAP]: https://en.wikipedia.org/wiki/Internet_Message_Access_Protocol
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org
