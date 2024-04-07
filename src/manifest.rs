use std::collections::HashMap;

use eyre::{Result, WrapErr};
use rune::{Any, Module, Source, Sources, Value};
use serde::Deserialize;

use crate::rune::init_rune_vm;
use crate::version::Version;

#[derive(Debug, Deserialize, Any)]
#[rune(constructor)]
pub struct Manifest {
    pub name: String,
    pub version: Version,
    // TODO: Add an Author type
    pub authors: Vec<String>,
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
    use super::*;

    #[test]
    fn minimal_deserializes_correctly_from_toml() {
        let toml = r#"
            name = "my-project"
            version = "0.1.0"
            authors = ["Alice", "Bob"]

            [templates]
        "#;

        let manifest = Manifest::from_toml(toml).unwrap();

        assert_eq!(manifest.name, "my-project");
        assert_eq!(manifest.version, Version::new(0, 1, 0));
        assert_eq!(
            manifest.authors,
            vec!["Alice".to_string(), "Bob".to_string()]
        );
    }

    #[test]
    fn minimal_deserializes_correctly_from_yaml() {
        let yaml = r#"
            name: my-project
            version: "0.1.0"
            authors: ["Alice", "Bob"]
            templates: {}
        "#;

        let manifest = Manifest::from_yaml(yaml).unwrap();

        assert_eq!(manifest.name, "my-project");
        assert_eq!(manifest.version, Version::new(0, 1, 0));
        assert_eq!(
            manifest.authors,
            vec!["Alice".to_string(), "Bob".to_string()]
        );
    }

    #[test]
    fn minimal_deserializes_correctly_from_json() {
        let json = r#"
            {
                "name": "my-project",
                "version": "0.1.0",
                "authors": ["Alice", "Bob"],
                "templates": {}
            }
        "#;

        let manifest = Manifest::from_json(json).unwrap();

        assert_eq!(manifest.name, "my-project");
        assert_eq!(manifest.version, Version::new(0, 1, 0));
        assert_eq!(
            manifest.authors,
            vec!["Alice".to_string(), "Bob".to_string()]
        );
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
                    authors: ["Alice", "Bob"],
                    templates: #{}
                })
            }
        "#,
        )
        .unwrap();

        let manifest = Manifest::from_rune(source).unwrap();

        assert_eq!(manifest.name, "my-project");
        assert_eq!(manifest.version, Version::new(0, 1, 0));
        assert_eq!(
            manifest.authors,
            vec!["Alice".to_string(), "Bob".to_string()]
        );
    }
}
