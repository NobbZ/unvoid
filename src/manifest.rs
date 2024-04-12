use std::collections::HashMap;

use eyre::{Result, WrapErr};
use rune::{Any, Module, Source, Sources, Value};
use serde::Deserialize;

use crate::author::Author;
use crate::rune::init_rune_vm;
use crate::rune::ty::manifest::Manifest as RuneManifest;
use crate::version::Version;

#[derive(Debug, Deserialize, Any)]
#[rune(constructor)]
pub struct Manifest {
    pub name: String,
    pub version: Version,
    pub authors: Vec<Author>,
    // TODO: Add a template type
    pub templates: HashMap<String, ()>,
}

impl Manifest {
    pub fn from_toml<S>(toml: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        toml::from_str(toml.as_ref()).wrap_err("Can not parse manifest from TOML")
    }

    pub fn from_yaml<S>(yaml: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        serde_yaml::from_str(yaml.as_ref()).wrap_err("Can not parse manifest from YAML")
    }

    pub fn from_json<S>(json: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        serde_json::from_str(json.as_ref()).wrap_err("Can not parse manifest from JSON")
    }

    pub fn from_rune(source: Source) -> Result<Self> {
        let name = source.name().to_string();

        let mut sources = Sources::new();
        sources
            .insert(source)
            .wrap_err_with(|| format!("unable to insert source '{}'", name))?;

        let mut vm = init_rune_vm(&mut sources)?;

        let output = vm
            .call(["manifest"], ())
            .wrap_err("Unable to call manifest entrypoint")?;

        let result: Result<Value, String> = rune::from_value(output)?;

        match result {
            Ok(value) => Ok(rune::from_value(value)?),
            Err(err) => Err(eyre::eyre!(err)),
        }
    }

    pub fn register(module: &mut Module) -> Result<()> {
        module.ty::<Manifest>()?;

        Ok(())
    }
}

impl From<RuneManifest> for Manifest {
    fn from(value: RuneManifest) -> Self {
        Self {
            name: value.name,
            version: value.version.into(),
            authors: value.authors.into_iter().map(Author::from).collect(),
            templates: value.templates,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rune::ty::manifest::Manifest as RuneManifest;
    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use rune::sources;

    lazy_static! {
        static ref ALICE: Author = Author::String("Alice".into());
        static ref BOB: Author = Author::Structured {
            name: "Bob".into(),
            email: None,
        };
        static ref CHARLIE: Author = Author::Structured {
            name: "Charlie".into(),
            email: Some("example@example.com".into()),
        };
    }

    macro_rules! assert_manifest {
        ($manifest:expr) => {
            assert_eq!($manifest.name, "my-project");
            assert_eq!($manifest.version, Version::new(0, 1, 0));
            assert_eq!(
                $manifest.authors,
                vec![ALICE.clone(), BOB.clone(), CHARLIE.clone()]
            );
        };
    }

    #[test]
    fn minimal_deserializes_correctly_from_toml() {
        let toml = r#"
            name = "my-project"
            version = "0.1.0"
            authors = [
                "Alice",
                {name = "Bob"},
                {name = "Charlie", email = "example@example.com"}
            ]

            [templates]
        "#;

        let manifest = Manifest::from_toml(toml).unwrap();

        assert_manifest!(manifest);
    }

    #[test]
    fn minimal_deserializes_correctly_from_yaml() {
        let yaml = r#"
            name: my-project
            version: "0.1.0"
            authors:
            - "Alice"
            - name: "Bob"
            - name: "Charlie"
              email: "example@example.com"
            templates: {}
        "#;

        let manifest = Manifest::from_yaml(yaml).unwrap();

        assert_manifest!(manifest);
    }

    #[test]
    fn minimal_deserializes_correctly_from_json() {
        let json = r#"
            {
                "name": "my-project",
                "version": "0.1.0",
                "authors": [
                    "Alice",
                    {"name": "Bob"},
                    {"name": "Charlie", "email": "example@example.com"}
                ],
                "templates": {}
            }
        "#;

        let manifest = Manifest::from_json(json).unwrap();

        assert_manifest!(manifest);
    }

    #[test]
    fn minimal_deserializes_correctly_from_rune() -> Result<()> {
        let mut sources = sources! {
            memory => {
                use ::unvoid::{
                    author::Author,
                    manifest::Manifest,
                    version::Version,
                };

                pub fn manifest() {
                    let version = Version::parse_simple("0.1.0").unwrap();

                    Manifest {
                        name: "my-project",
                        version: version,
                        authors: [
                            Author::new("Alice"),
                            Author::new("Bob"),
                            Author::new("Charlie").with_email("example@example.com"),
                        ],
                        templates: #{}
                    }
                }
            }
        };

        let rune_manifest: RuneManifest =
            rune::from_value(init_rune_vm(&mut sources)?.call(["manifest"], ())?)?;

        let manifest: Manifest = rune_manifest.into();

        assert_manifest!(manifest);

        Ok(())
    }

    #[test]
    fn minimal_seserializes_correctly_from_rune_file() -> Result<()> {
        let source = Source::from_path("tests/manifests/simple.rn")?;
        let mut sources = Sources::new();
        sources.insert(source)?;

        let rune_manifest: RuneManifest =
            rune::from_value(init_rune_vm(&mut sources)?.call(["manifest"], ())?)?;

        let manifest: Manifest = rune_manifest.into();

        assert_manifest!(manifest);

        Ok(())
    }
}
