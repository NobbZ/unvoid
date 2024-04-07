use std::collections::HashMap;

use eyre::{Result, WrapErr};
use rune::{Any, Module, Source, Sources, Value};
use serde::Deserialize;

use crate::author::Author;
use crate::rune::init_rune_vm;
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

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::*;

    use pretty_assertions::assert_eq;

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
    fn minimal_deserializes_correctly_from_rune() {
        let source = Source::memory(
            r#"
            pub fn manifest() {
                let version = Version::parse("0.1.0")?;

                Ok(Manifest {
                    name: "my-project",
                    version: version,
                    authors: [
                        Author::from_str("Alice"),
                        Author::new("Bob"),
                        Author::with_mail("Charlie", "example@example.com"),
                    ],
                    templates: #{}
                })
            }
        "#,
        )
        .unwrap();

        let manifest = Manifest::from_rune(source).unwrap();

        assert_manifest!(manifest);
    }

    #[test]
    fn minimal_seserializes_correctly_from_rune_file() {
        let source = Source::from_path("tests/manifests/simple.rn").unwrap();

        let manifest = Manifest::from_rune(source).unwrap();

        assert_manifest!(manifest);
    }
}
