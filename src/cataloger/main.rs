// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-imap-cataloger requires the 'std' feature");

use asimov_imap_module::{ImapConfiguration, ImapOrderBy, ImapReader};
use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use dogma::{
    Uri,
    UriScheme::{Imap, Imaps},
    UriValueParser,
};
use std::{error::Error, io::stdout};

/// asimov-imap-cataloger
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Order messages by a property.
    #[arg(
        value_name = "PROPERTY",
        short = 'b',
        long,
        alias = "order",
        default_value_t,
        value_enum
    )]
    order_by: ImapOrderBy,

    /// Limit the number of messages to catalog.
    #[arg(value_name = "COUNT", short = 'n', long)]
    limit: Option<usize>,

    /// Set the output format [default: cli] [possible values: cli, json, jsonld, jsonl]
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    /// An `imaps://user@host:port/mailbox` (or `imap://...`) URL to the IMAP mailbox to catalog.
    #[arg(value_name = "IMAP-MAILBOX-URL", value_parser = UriValueParser::new(&[Imap, Imaps]))]
    mailbox_url: Uri<'static>,
}

fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    // Resolve the authenticated URL:
    let config = ImapConfiguration::load()?;
    let imap_url = config.resolve_url((&options.mailbox_url).into())?;

    // Connect to the IMAP server:
    let mut mailbox = ImapReader::open(&imap_url)?;

    // Scan the mailbox messages:
    let messages = mailbox.iter(options.order_by, options.limit)?;
    let messages = messages.take(options.limit.unwrap_or(usize::MAX));
    match options
        .output
        .as_ref()
        .unwrap_or(&String::default())
        .as_str()
    {
        "jsonl" => {
            use know::traits::ToJsonLd;
            for message in messages {
                let message = message?;
                let json = message.headers.to_jsonld()?;
                serde_json::to_writer(stdout(), &json)?;
                println!();
            }
        },
        "jsonld" | "json" => {
            use know::traits::ToJsonLd;
            let mut output = Vec::new();
            for message in messages {
                let message = message?;
                output.push(message.headers.to_jsonld()?);
            }
            if cfg!(feature = "pretty") {
                colored_json::write_colored_json(&output, &mut stdout())?;
            } else {
                serde_json::to_writer_pretty(stdout(), &output)?;
            }
            println!();
        },
        "cli" | _ => {
            for (index, message) in messages.enumerate() {
                let message = message?;
                if index > 0 && options.flags.verbose > 0 {
                    println!();
                }
                match options.flags.verbose {
                    0 => print!("{}", message.headers.oneliner()),
                    1 => print!("{}", message.headers.concise()),
                    _ => print!("{}", message.headers.detailed()),
                }
            }
        },
    }

    Ok(EX_OK)
}
