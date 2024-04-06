use std::collections::HashMap;
use std::sync::Arc;

use eyre::{Result, WrapErr};
use rune::runtime::VmResult;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, FromValue, Source, Sources, Value, Vm};
use semver::Version;
use serde::Deserialize;

macro_rules! try_from_value {
    ($val:expr) => {
        match rune::from_value($val) {
            Ok(val) => val,
            Err(err) => return VmResult::Err(err),
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub version: Version,
    // TODO: Add an Author type
    pub authors: Vec<String>,
    // TODO: Add a template type
    pub templates: HashMap<String, ()>,
}

macro_rules! try_from_value {
    ($val:expr) => {
        match rune::from_value($val) {
            Ok(val) => val,
            Err(err) => return VmResult::Err(err),
        }
    };
}

impl FromValue for Manifest {
    fn from_value(value: Value) -> VmResult<Self> {
        let map: HashMap<String, Value> = try_from_value!(value);
        let name: String = try_from_value!(map.get("name").unwrap().clone());
        let authors: Vec<String> = try_from_value!(map.get("authors").unwrap().clone());
        let templates: HashMap<String, ()> = try_from_value!(map.get("templates").unwrap().clone());

        let version_str: String = try_from_value!(map.get("version").unwrap().clone());
        let version: Version = Version::parse(&version_str).unwrap();

        VmResult::Ok(Self {
            name,
            version,
            authors,
            templates,
        })
    }
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
        let ctx = Context::with_default_modules().wrap_err("Unable to initialize rune context")?;
        let rt = Arc::new(
            ctx.runtime()
                .wrap_err("Unable to initialize rune runtime")?,
        );

        let name = source.name().to_string();

        let mut sources = Sources::new();
        sources
            .insert(source)
            .wrap_err_with(|| format!("unable to insert source '{}'", name))?;

        let mut diag = Diagnostics::new();

        let result = rune::prepare(&mut sources)
            .with_context(&ctx)
            .with_diagnostics(&mut diag)
            .build();

        if !diag.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diag.emit(&mut writer, &sources)?;
        }

        let unit = result.wrap_err("unable to build rune unit")?;
        let mut vm = Vm::new(rt, Arc::new(unit));

        let output = vm
            .call(["manifest"], ())
            .wrap_err("Unable to call manifest entrypoint")?;

        rune::from_value(output).wrap_err("Unable to convert rune value to manifest")
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
                #{
                    name: "my-project",
                    version: "0.1.0",
                    authors: ["Alice", "Bob"],
                    templates: #{}
                }
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
