use std::ops::Deref;

use rune::{runtime::VmResult, FromValue, Value};
use semver::Version as SemVer;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Version(SemVer);

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        SemVer::new(major, minor, patch).into()
    }
}

impl FromValue for Version {
    fn from_value(value: Value) -> VmResult<Self> {
        let version_str: String = match FromValue::from_value(value) {
            VmResult::Ok(version_str) => version_str,
            VmResult::Err(err) => return VmResult::Err(err),
        };

        let version = SemVer::parse(&version_str).unwrap();

        VmResult::Ok(Version(version))
    }
}

impl From<SemVer> for Version {
    fn from(semver: SemVer) -> Self {
        Version(semver)
    }
}

impl Deref for Version {
    type Target = SemVer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<SemVer> for Version {
    fn as_ref(&self) -> &SemVer {
        &self.0
    }
}
