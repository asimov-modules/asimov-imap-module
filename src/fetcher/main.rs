// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-imap-fetcher requires the 'std' feature");

use asimov_imap_module::{ImapConfiguration, ImapReader};
use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use dogma::{
    Uri,
    UriScheme::{Imap, Imaps},
    UriValueParser,
};
use know::datatypes::EmailMessageId;
use std::error::Error;

/// asimov-imap-fetcher
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The output format.
    #[arg(id = "FORMAT", short = 'o', long)]
    output_format: Option<String>,

    /// An `imaps://user@host:port/mailbox#mid` (or `imap://...`) URL to the message to fetch.
    #[arg(id = "IMAP-MESSAGE-URL", value_parser = UriValueParser::new(&[Imap, Imaps]))]
    message_url: Uri<'static>,
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
    let imap_url = config.resolve_url((&options.message_url).into())?;

    // Connect to the IMAP server:
    let mut server = ImapReader::open(&imap_url)?;

    // Fetch the IMAP message:
    let message_id: EmailMessageId = options
        .message_url
        .fragment_str()
        .expect("message ID should be given in the URL fragment")
        .into();
    match server.fetch(&message_id)? {
        Some(message) => {
            match options.output_format.unwrap_or_default().as_str() {
                "jsonld" | "json" => print!("{}", message.headers.jsonld()),
                "mime" | _ => {
                    print!("{}", message.headers.mime());
                    if let Some(body) = message.body {
                        println!();
                        print!("{}", body);
                    }
                },
            }
            Ok(EX_OK)
        },
        None => {
            eprintln!("message ID <{}> not found", message_id.as_str());
            Ok(EX_NOINPUT)
        },
    }
}
