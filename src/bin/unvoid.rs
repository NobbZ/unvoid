use color_eyre::Result;
use tracing::{info, Level};
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

use unvoid::interface::UVParser;

fn main() -> Result<()> {
    let args = <UVParser as clap::Parser>::parse();

    let (level, fmt_span) = if args.verbose {
        (Level::TRACE, FmtSpan::NEW | FmtSpan::CLOSE)
    } else {
        (Level::INFO, FmtSpan::NONE)
    };

    let sub = FmtSubscriber::builder()
        .with_max_level(level)
        .with_span_events(fmt_span)
        .finish();

    tracing::subscriber::set_global_default(sub).expect("setting default subscriber failed");

    tokio::runtime::Runtime::new()?.block_on(async_main(args))
}

#[tracing::instrument(name = "main", level = "trace")]
async fn async_main(args: UVParser) -> Result<()> {
    info!("Hello, world!");

    Ok(())
}
