use serde::Deserialize;

use crate::rune::ty::author::Author as RuneAuthor;

#[derive(Debug, Deserialize,  Clone)]
#[serde(untagged)]
pub enum Author {
    String(String),
    Structured { name: String, email: Option<String> },
}

impl Author {
    pub fn get_name(&self) -> String {
        match self {
            Self::String(name) => name.clone(),
            Self::Structured { name, .. } => name.clone(),
        }
    }

    pub fn get_email(&self) -> Option<String> {
        match self {
            Self::Structured { email, .. } => email.clone(),
            _ => None,
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
