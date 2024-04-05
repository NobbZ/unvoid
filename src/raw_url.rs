use std::{fmt::Debug, str::FromStr};

use eyre::{Report, Result};
use url::Url;

#[derive(Clone)]
pub struct RawUrl {
    pub raw: String,
    pub final_url: Url,
}

impl FromStr for RawUrl {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        match Url::parse(s) {
            Ok(url) => Ok(RawUrl {
                final_url: url,
                raw: s.to_string(),
            }),
            Err(parse_error) => maybe_fix_url_base(s, &parse_error),
        }
    }
}

impl Debug for RawUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let final_url = format!("<{}>", self.final_url);

        f.debug_struct("RawUrl")
            .field("raw", &self.raw)
            .field("final_url", &final_url)
            .finish()
    }
}

fn maybe_fix_url_base(url_str: &str, parse_error: &url::ParseError) -> Result<RawUrl> {
    let pwd = std::env::current_dir()?;
    let base_url = Url::parse(&format!("file://{}/", pwd.display()))?;

    match parse_error {
        url::ParseError::RelativeUrlWithoutBase => Ok(RawUrl {
            raw: url_str.into(),
            final_url: base_url.join(url_str)?,
        }),
        &e => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("/foo/bar", "file:///foo/bar")]
    #[case("/foo/bar/", "file:///foo/bar/")]
    #[case("foo/bar/", &format!("file://{}/foo/bar/", std::env::current_dir().unwrap().display()))]
    #[case("foo/bar", &format!("file://{}/foo/bar", std::env::current_dir().unwrap().display()))]
    #[case(".", &format!("file://{}/", std::env::current_dir().unwrap().display()))]
    #[case("./", &format!("file://{}/", std::env::current_dir().unwrap().display()))]
    #[case("./.", &format!("file://{}/", std::env::current_dir().unwrap().display()))]
    fn resolves_paths_correctly(#[case] input: &str, #[case] expected: &str) {
        let raw_url: RawUrl = input.parse().unwrap();

        assert_eq!(raw_url.final_url.to_string(), expected);
    }

    #[rstest]
    #[case("/foo/bar")]
    #[case("/foo/bar/")]
    #[case("foo/bar/")]
    #[case("foo/bar")]
    #[case(".")]
    #[case("./")]
    #[case("./.")]
    fn raw_is_preserved_correctly(#[case] input: &str) {
        let raw_url: RawUrl = input.parse().unwrap();

        assert_eq!(raw_url.raw, input);
    }
}
