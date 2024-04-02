use clap::{crate_authors, Parser};

use crate::raw_url::RawUrl;

const ABOUT: &str = "A tool that helps bootstrapping your projects";

#[derive(Parser, Debug)]
#[command(version, about = ABOUT, author = crate_authors!(), long_about = None)]
pub struct UVParser {
    /// Enable verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// The template to use
    #[arg(default_value = ".")]
    pub template: RawUrl,
}
