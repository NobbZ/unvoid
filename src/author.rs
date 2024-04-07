use eyre::Result;
use rune::{Any, Module};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Any, Clone)]
#[serde(untagged)]
#[rune(constructor)]
pub enum Author {
    String(String),
    Structured { name: String, email: Option<String> },
}

impl Author {
    #[rune::function(path = Author::from_str)]
    fn rune_from_str(s: String) -> Self {
        Self::String(s)
    }

    #[rune::function(path = Author::new)]
    fn rune_new(name: String) -> Self {
        Self::Structured { name, email: None }
    }

    #[rune::function(path = Author::with_mail)]
    fn rune_with_mail(name: String, email: String) -> Self {
        Self::Structured {
            name,
            email: Some(email),
        }
    }

    pub fn register(module: &mut Module) -> Result<()> {
        module.ty::<Author>()?;

        module.function_meta(Self::rune_from_str)?;
        module.function_meta(Self::rune_new)?;
        module.function_meta(Self::rune_with_mail)?;

        Ok(())
    }
}

impl<S> From<S> for Author
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        Author::String(s.as_ref().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    fn toml_from_str(input: &str) -> HashMap<String, Author> {
        toml::from_str(input).unwrap()
    }

    fn yaml_from_str(input: &str) -> HashMap<String, Author> {
        serde_yaml::from_str(input).unwrap()
    }

    fn json_from_str(input: &str) -> HashMap<String, Author> {
        serde_json::from_str(input).unwrap()
    }

    #[rstest]
    #[case::toml_string("author = \"Alice\"", toml_from_str, Author::String("Alice".to_string()))]
    #[case::toml_structured(
        "author = { name = \"Alice\", email = \"example@example.com\" }",
        toml_from_str,
        Author::Structured {
            name: "Alice".to_string(),
            email: Some("example@example.com".into()),
        },
    )]
    #[case::yaml_string("author: \"Alice\"", yaml_from_str, Author::String("Alice".to_string()))]
    #[case::yaml_structured(
        "author: { name: \"Alice\", email: \"example@example.com\" }",
        yaml_from_str,
        Author::Structured {
            name: "Alice".to_string(),
            email: Some("example@example.com".into()),
        },
    )]
    #[case::json_string(r#"{"author": "Alice"}"#, json_from_str, Author::String("Alice".to_string()))]
    #[case::json_structured(
        r#"{"author": { "name": "Alice", "email": "example@example.com" }}"#,
        json_from_str,
        Author::Structured {
            name: "Alice".to_string(),
            email: Some("example@example.com".into()),
        },
    )]
    fn test_author_deserialisation(
        #[case] input: &str,
        #[case] from_str: fn(&str) -> HashMap<String, Author>,
        #[case] expected: Author,
    ) {
        let map: HashMap<String, Author> = from_str(input);
        let author = map.get("author").unwrap();

        assert_eq!(author, &expected);
    }
}
