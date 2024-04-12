use serde::Deserialize;

use crate::rune::ty::author::Author as RuneAuthor;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Author {
    String(String),
    Structured { name: String, email: Option<String> },
}

impl Author {
    pub fn get_name(&self) -> &str {
        match self {
            Self::String(name) => name,
            Self::Structured { name, .. } => name,
        }
    }

    pub fn get_email(&self) -> &Option<String> {
        match self {
            Self::Structured { email, .. } => email,
            _ => &None,
        }
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

impl From<RuneAuthor> for Author {
    fn from(rune_author: RuneAuthor) -> Self {
        Author::Structured {
            name: rune_author.name,
            email: rune_author.email,
        }
    }
}

impl PartialEq<Author> for Author {
    fn eq(&self, other: &Author) -> bool {
        self.get_name() == other.get_name() && self.get_email() == other.get_email()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::collections::HashMap;

    lazy_static::lazy_static! {
        static ref SIMPLE_AUTHOR: Author = Author::String("Alice".to_string());
        static ref EMAIL_AUTHOR: Author = Author::Structured {
            name: "Alice".to_string(),
            email: Some("example@example.com".into()),
        };

    }

    const TOML_SIMPLE: &str = r#"author = "Alice""#;
    const YAML_SIMPLE: &str = r#"author: "Alice""#;
    const JSON_SIMPLE: &str = r#"{"author": "Alice"}"#;

    const TOML_WITH_MAIL: &str = r#"author = { name = "Alice", email = "example@example.com" }"#;
    const YAML_WITH_MAIL: &str = r#"author: { name: "Alice", email: "example@example.com" }"#;
    const JSON_WITH_MAIL: &str =
        r#"{"author": { "name": "Alice", "email": "example@example.com" }}"#;

    fn str2toml(input: &str) -> HashMap<String, Author> {
        toml::from_str(input).unwrap()
    }

    fn str2yaml(input: &str) -> HashMap<String, Author> {
        serde_yaml::from_str(input).unwrap()
    }

    fn str2json(input: &str) -> HashMap<String, Author> {
        serde_json::from_str(input).unwrap()
    }

    #[rstest]
    #[case::toml_string(TOML_SIMPLE, str2toml, &SIMPLE_AUTHOR)]
    #[case::toml_structured(TOML_WITH_MAIL, str2toml, &EMAIL_AUTHOR)]
    #[case::yaml_string(YAML_SIMPLE, str2yaml, &SIMPLE_AUTHOR)]
    #[case::yaml_structured(YAML_WITH_MAIL, str2yaml, &EMAIL_AUTHOR)]
    #[case::json_string(JSON_SIMPLE, str2json, &SIMPLE_AUTHOR)]
    #[case::json_structured(JSON_WITH_MAIL, str2json, &EMAIL_AUTHOR)]
    fn test_author_deserialisation(
        #[case] input: &str,
        #[case] from_str: fn(&str) -> HashMap<String, Author>,
        #[case] expected: &Author,
    ) {
        let map: HashMap<String, Author> = from_str(input);
        let author = map.get("author").unwrap();

        assert_eq!(author, expected);
    }
}
