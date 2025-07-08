// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-imap-cataloger requires the 'std' feature");

use asimov_imap_module::ImapReader;
use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use dogma::{
    Uri,
    UriScheme::{Imap, Imaps},
    UriValueParser,
};
use std::error::Error;

/// asimov-imap-cataloger
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The URL of the IMAP server.
    #[arg(value_parser = UriValueParser::new(&[Imap, Imaps]))]
    url: Uri<'static>,
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

    // Connect to the IMAP server:
    let mut reader = ImapReader::open(&options.url)?;

    // Scan the mailbox messages:
    for (index, entry) in reader.iter()?.enumerate() {
        let email = entry?;
        if index > 0 {
            println!();
        }
        print!("{}", email.message);
    }

    Ok(EX_OK)
}
