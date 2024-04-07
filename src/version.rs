use std::ops::Deref;

use eyre::Result;
use rune::{runtime::Protocol, Any, Module};
use semver::Version as SemVer;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Any)]
pub struct Version(SemVer);

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        SemVer::new(major, minor, patch).into()
    }

    #[rune::function(path = Self::new)]
    fn new_version(major: u64, minor: u64, patch: u64) -> Self {
        Self::new(major, minor, patch)
    }

    pub fn set_major(&mut self, major: u64) {
        self.0.major = major;
    }

    pub fn set_minor(&mut self, minor: u64) {
        self.0.minor = minor;
    }

    pub fn set_patch(&mut self, patch: u64) {
        self.0.patch = patch;
    }

    pub fn get_major(&self) -> u64 {
        self.0.major
    }

    pub fn get_minor(&self) -> u64 {
        self.0.minor
    }

    pub fn get_patch(&self) -> u64 {
        self.0.patch
    }

    pub fn register(module: &mut Module) -> Result<()> {
        module.ty::<Version>()?;

        module.function_meta(Self::new_version)?;

        module.field_function(Protocol::SET, "major", Self::set_major)?;
        module.field_function(Protocol::SET, "minor", Self::set_minor)?;
        module.field_function(Protocol::SET, "patch", Self::set_patch)?;

        module.field_function(Protocol::GET, "major", Self::get_major)?;
        module.field_function(Protocol::GET, "minor", Self::get_minor)?;
        module.field_function(Protocol::GET, "patch", Self::get_patch)?;

        Ok(())
    }
}

// impl FromValue for Version {
//     fn from_value(value: Value) -> VmResult<Self> {
//         let version_str: String = match FromValue::from_value(value) {
//             VmResult::Ok(version_str) => version_str,
//             VmResult::Err(err) => return VmResult::Err(err),
//         };

//         let version = SemVer::parse(&version_str).unwrap();

//         VmResult::Ok(Version(version))
//     }
// }

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