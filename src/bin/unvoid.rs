use color_eyre::Result;
use tracing::{info, Level};
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

use unvoid::interface::UVParser;

#[cfg(debug_assertions)]
const VERBOSE_LEVEL: Level = Level::TRACE;
#[cfg(not(debug_assertions))]
const VERBOSE_LEVEL: Level = Level::DEBUG;

#[cfg(debug_assertions)]
const EXTREMELY_VERBOSE: bool = true;
#[cfg(not(debug_assertions))]
const EXTREMELY_VERBOSE: bool = false;

fn main() -> Result<()> {
    let args = <UVParser as clap::Parser>::parse();

    FmtSubscriber::builder()
        .with_max_level(get_level(&args))
        .with_span_events(get_span(&args))
        .with_file(args.verbose && EXTREMELY_VERBOSE)
        .with_line_number(args.verbose && EXTREMELY_VERBOSE)
        .init();

    tracing::debug!(?args, "Parsed arguments");

    tokio::runtime::Runtime::new()?.block_on(async_main(args))
}

#[tracing::instrument(name = "main", level = "trace", skip(args))]
async fn async_main(args: UVParser) -> Result<()> {
    info!(template = %args.template, "getting template");

    Ok(())
}

fn get_level(args: &UVParser) -> Level {
    if args.verbose {
        VERBOSE_LEVEL
    } else {
        Level::INFO
    }
}

fn get_span(args: &UVParser) -> FmtSpan {
    if args.verbose {
        FmtSpan::NEW | FmtSpan::CLOSE
    } else {
        FmtSpan::NONE
    }
}
