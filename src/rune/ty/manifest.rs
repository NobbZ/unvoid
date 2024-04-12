use std::collections::HashMap;

use eyre::Result;
use rune::{Any, Module};

use super::{author::Author, version::Version};

#[derive(Debug, Any)]
#[rune(constructor, item = ::unvoid::manifest)]
pub struct Manifest {
    #[rune(get, set)]
    pub name: String,
    #[rune(get, set)]
    pub version: Version,
    #[rune(get, set)]
    pub authors: Vec<Author>,
    // TODO: Add a template type
    #[rune(get, set)]
    pub templates: HashMap<String, ()>,
}

#[rune::module(::unvoid::manifest)]
pub fn module() -> Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;
    module.ty::<Manifest>()?;

    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rune::init_rune_vm;
    use eyre::Result;
    use pretty_assertions::assert_eq;
    use rune::sources;

    #[test]
    fn test_simple_manifest() -> Result<()> {
        let mut sources = sources! {
            manifest => {
                use ::unvoid::{manifest::Manifest, version::Version};

                pub fn manifest() {
                    Manifest {
                        name: "test-manifest",
                        version: Version::new(1,2,3),
                        authors: [],
                        templates: #{},
                    }
                }
            }
        };

        let manifest: Manifest =
            rune::from_value(init_rune_vm(&mut sources)?.call(["manifest"], ())?)?;

        assert_eq!(manifest.name, "test-manifest");
        assert_eq!(manifest.version.major, 1);
        assert_eq!(manifest.version.minor, 2);
        assert_eq!(manifest.version.patch, 3);
        assert_eq!(manifest.authors, Vec::new());
        assert_eq!(manifest.templates, HashMap::new());

        Ok(())
    }
}
