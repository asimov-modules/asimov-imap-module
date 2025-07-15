# ASIMOV IMAP Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-imap-module)](https://crates.io/crates/asimov-imap-module)
[![Documentation](https://docs.rs/asimov-imap-module/badge.svg)](https://docs.rs/asimov-imap-module)

[ASIMOV] module for [IMAP] email import.

## ✨ Features

- Fetches email messages from IMAP servers and outputs them as [JSON-LD].
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports Gmail, Outlook, Yahoo, iCloud, Proton Mail, GMX, Fastmail, and just
  about any other [email provider](#cloud-email-providers).
- Uses server-side sorting of email messages with servers that support it.
- Implements optimal client-side sorting when server-side sorting isn't available.
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install imap -v
```

![Installation with the ASIMOV !CLI](https://github.com/asimov-modules/asimov-imap-module/blob/master/etc/install.svg)

### Installation from Source Code

```bash
cargo install asimov-imap-module
```

## 👉 Examples

### Email Import from IETF Mailing Lists

#### Cataloging email messages on a mailing list

```bash
asimov list imaps://imap.ietf.org/Shared%20Folders/json-canon
```

![Cataloging email messages on a mailing list](https://github.com/asimov-modules/asimov-imap-module/blob/master/etc/catalog.svg)

### Email Import from Gmail

#### Cataloging email messages in the inbox

```bash
asimov-imap-cataloger imaps://imap.gmail.com/INBOX -n5
```

#### Fetching a specific email message

```bash
asimov-imap-fetcher imaps://imap.gmail.com/INBOX#mid
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

### Gmail Configuration

To connect to your Gmail (aka Google Mail) account, follow these steps:

1. [Enable 2-Step Verification](https://support.google.com/accounts/answer/185839)
   in your account's [Security](https://myaccount.google.com/security)
   configuration.
2. [Create an app password](https://support.google.com/accounts/answer/185833)
   in your account's [App passwords](https://myaccount.google.com/apppasswords)
   configuration. (You can enter any app name, such as "ASIMOV".)

Then, construct your IMAP credentials and URL as follows:

- Use the `imaps:` URL scheme for a secure connection.
- Use your full email address--including `@gmail.com`, `@googlemail.com`,
  or your own Google Workspace domain name--as the IMAP username.
- Use the app password your created as the IMAP password.
- Use `imap.gmail.com` for the IMAP hostname.
- Use 993 for the IMAP port, or just omit the port altogether.

The authentication credentials can be configured using any of the
[aforementioned](#authentication-credentials) methods.

For example, configure the `~/.netrc` (aka `$HOME/.netrc`) file to store your
Gmail credentials as follows:

```
machine imap.gmail.com
login myuser@gmail.com
password myapppassword
```

Test your configuration by attempting to list the first five email messages in
your inbox:

```bash
asimov-imap-cataloger imaps://imap.gmail.com/INBOX -n5
```

## 📚 Reference

### Cloud Email Providers

| Provider | Protocol | Username | Hostname | Port |
| :------- | :------- | :------- | :------- | :--- |
| Alibaba Mail | `imap:` | `myuser@alibaba.com` | [`imap.alibaba.com`] | 143 |
| AOL Mail | `imaps:` | `myuser@aol.com` | [`imap.aol.com`] | 993 |
| Fastmail | `imaps:` | `myuser@fastmail.com` | [`imap.fastmail.com`] | 993 |
| GMX Mail | `imaps:` | `myuser@gmx.com` | [`imap.gmx.com`] | 993 |
| Gmail (Google Mail) | `imaps:` | `myuser@gmail.com` | [`imap.gmail.com`] | 993 |
| iCloud Mail | `imaps:` | `myuser@icloud.com` | [`imap.mail.me.com`] | 993 |
| Mail.com | `imaps:` | `myuser@mail.com` | [`imap.mail.com`] | 993 |
| NetEase Mail (163) | `imaps:` | `myuser@163.com` | [`imap.163.com`] | 993 |
| NetEase Mail (126) | `imaps:` | `myuser@126.com` | [`imap.126.com`] | 993 |
| Outlook | `imaps:` | `myuser@outlook.com` | [`outlook.office365.com`] | 993 |
| Proton Mail | `imaps:` | `myuser@proton.me` | `127.0.0.1` | 1143 |
| QQ Mail | `imaps:` | `myuser@qq.com` | [`imap.qq.com`] | 993 |
| Sina Mail | `imaps:` | `myuser@sina.com` | [`imap.sina.com`] | 993 |
| Sohu Mail | `imaps:` | `myuser@sohu.com` | [`imap.sohu.com`] | 993 |
| Yahoo Mail | `imaps:` | `myuser@yahoo.com` | [`imap.mail.yahoo.com`] | 993 |
| Zoho Mail | `imaps:` | `myuser@zoho.com` | [`imap.zoho.com`] | 993 |

### `asimov-imap-cataloger`

```
asimov-imap-cataloger

Usage: asimov-imap-cataloger [OPTIONS] <IMAP-MAILBOX-URL>

Arguments:
  <IMAP-MAILBOX-URL>  An `imaps://user@host:port/mailbox` (or `imap://...`) URL to the IMAP mailbox to catalog

Options:
  -d, --debug                Enable debugging output
      --license              Show license information
  -v, --verbose...           Enable verbose output (may be repeated for more verbosity)
  -V, --version              Print version information
  -b, --order-by <PROPERTY>  The message property to order messages by [default: none] [possible values: none, timestamp, date, from, to, cc, size]
  -n, --limit <COUNT>        The maximum number of messages to catalog
  -o, --output <FORMAT>      The output format
  -h, --help                 Print help (see more with '--help')
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

[`imap.126.com`]: https://help.mail.126.com/faqDetail.do?code=d7a5dc8471cd0c0e8b4b8f4f8e49998b374173cfe9171305fa1ce630d7f67ac24aac98d1012d23f2
[`imap.163.com`]: https://help.mail.163.com/faqDetail.do?code=d7a5dc8471cd0c0e8b4b8f4f8e49998b374173cfe9171305fa1ce630d7f67ac24aac98d1012d23f2
[`imap.alibaba.com`]: https://so.alibaba.com/s/cgs/knowledge?categoryId=93847011&language=zh_CN&m_station=cgs&questionId=dc256e66a1064c65aee1de6f5095bde2
[`imap.aol.com`]: https://help.aol.com/articles/how-do-i-use-other-email-applications-to-send-and-receive-my-aol-mail
[`imap.fastmail.com`]: https://www.fastmail.help/hc/en-us/articles/1500000279921-IMAP-POP-and-SMTP
[`imap.gmail.com`]: https://support.google.com/mail/answer/7126229
[`imap.gmx.com`]: https://support.gmx.com/pop-imap/imap/server.html
[`imap.mail.com`]: https://support.mail.com/premium/imap/server.html
[`imap.mail.me.com`]: https://support.apple.com/en-us/102525
[`imap.mail.yahoo.com`]: https://help.yahoo.com/kb/SLN4075.html
[`imap.qq.com`]: https://service.mail.qq.com/detail/128/339
[`imap.sina.com`]: https://help.sina.com.cn/comquestiondetail/view/1565/
[`imap.sina.com`]: https://help.sina.com.cn/comquestiondetail/view/1565/
[`imap.sohu.com`]: https://cloud.tencent.com/developer/article/1800257
[`imap.zoho.com`]: https://www.zoho.com/mail/help/imap-access.html
[`outlook.office365.com`]: https://support.microsoft.com/en-us/office/pop-imap-and-smtp-settings-for-outlook-com-d088b986-291d-42b8-9564-9c414e2aa040
