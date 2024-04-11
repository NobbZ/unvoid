use rune::{Any, ContextError, Module};
use semver::Version as SemVer;

#[derive(Debug, PartialEq, Any)]
#[rune(item = ::unvoid::version)]
pub struct Version {
    #[rune(get, set)]
    pub major: u64,
    #[rune(get, set)]
    pub minor: u64,
    #[rune(get, set)]
    pub patch: u64,
    // pub pre: Prerelease,
    // pub build: BuildMetadata
}

impl Version {
    #[rune::function(path = Self::new)]
    fn rune_new(major: u64, minor: u64, patch: u64) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }

    /// Parses any valid semantic version into the simplified `Version`.
    ///
    /// Any existing pre-release or build info gets discarded.
    #[rune::function(path = Self::parse_simple)]
    fn rune_from_str(version: String) -> Result<Self, String> {
        let semver = SemVer::parse(&version).map_err(|err| err.to_string())?;

        Ok(Version {
            major: semver.major,
            minor: semver.minor,
            patch: semver.patch,
        })
    }
}

#[rune::module(::unvoid::version)]
// pub fn register(_module: &mut Module) -> Result<Module> {
pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::from_meta(self::module_meta)?;
    module.ty::<Version>()?;

    module.function_meta(Version::rune_new)?;
    module.function_meta(Version::rune_from_str)?;

    Ok(module)
}

impl From<SemVer> for Version {
    fn from(version: SemVer) -> Self {
        Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
        }
    }
}

impl From<Version> for SemVer {
    fn from(version: Version) -> Self {
        SemVer::new(version.major, version.minor, version.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rune::init_rune_vm;
    use eyre::Result;
    use pretty_assertions::assert_eq;
    use rune::sources;

    #[test]
    fn test_version_new() -> Result<()> {
        let mut sources = sources! {
            new => {
                use ::unvoid::version::Version;

                pub fn main() {
                    Version::new(1, 2, 3)
                }
            }
        };

        let version: Version = rune::from_value(
            init_rune_vm(&mut sources)
                .unwrap()
                .call(["main"], ())
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            version,
            Version {
                major: 1,
                minor: 2,
                patch: 3,
            }
        );

        Ok(())
    }

    #[test]
    fn test_version_parse_simple() -> Result<()> {
        let mut sources = sources! {
            new => {
                use ::unvoid::version::Version;

                pub fn main() {
                    Version::parse_simple("3.2.1").unwrap()
                }
            }
        };

        let version: Version = rune::from_value(
            init_rune_vm(&mut sources)
                .unwrap()
                .call(["main"], ())
                .unwrap(),
        )
        .unwrap();

        assert_eq!(
            version,
            Version {
                major: 3,
                minor: 2,
                patch: 1,
            }
        );

        Ok(())
    }
}
