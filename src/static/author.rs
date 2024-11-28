use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Author {
    String(String),
    Structured {
        name: String,
        email: Option<String>,
        homepage: Option<String>,
        github: Option<String>,
    },
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

    pub fn get_homepage(&self) -> &Option<String> {
        match self {
            Self::Structured { homepage, .. } => homepage,
            _ => &None,
        }
    }

    pub fn get_github(&self) -> &Option<String> {
        match self {
            Self::Structured { github, .. } => github,
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

impl PartialEq<Author> for Author {
    fn eq(&self, other: &Author) -> bool {
        self.get_name() == other.get_name()
            && self.get_email() == other.get_email()
            && self.get_homepage() == other.get_homepage()
            && self.get_github() == other.get_github()
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
            homepage: None,
            github: None,
        };
        static ref HOMEPAGE_AUTHOR: Author = Author::Structured {
            name: "Alice".to_string(),
            email: None,
            homepage: Some("https://example.com".into()),
            github: None,
        };
        static ref GITHUB_AUTHOR: Author = Author::Structured {
            name: "Alice".to_string(),
            email: None,
            homepage: None,
            github: Some("example".into()),
        };
    }

    const TOML_SIMPLE: &str = r#"author = "Alice""#;
    const YAML_SIMPLE: &str = r#"author: "Alice""#;
    const JSON_SIMPLE: &str = r#"{"author": "Alice"}"#;

    const TOML_WITH_MAIL: &str = r#"author = { name = "Alice", email = "example@example.com" }"#;
    const YAML_WITH_MAIL: &str = r#"author: { name: "Alice", email: "example@example.com" }"#;
    const JSON_WITH_MAIL: &str =
        r#"{"author": { "name": "Alice", "email": "example@example.com" }}"#;

    const TOML_WITH_HOMEPAGE: &str =
        r#"author = { name = "Alice", homepage = "https://example.com" }"#;
    const YAML_WITH_HOMEPAGE: &str =
        r#"author: { name: "Alice", homepage: "https://example.com" }"#;
    const JSON_WITH_HOMEPAGE: &str =
        r#"{"author": { "name": "Alice", "homepage": "https://example.com" }}"#;

    const TOML_WITH_GITHUB: &str = r#"author = { name = "Alice", github = "example" }"#;
    const YAML_WITH_GITHUB: &str = r#"author: { name: "Alice", github: "example" }"#;
    const JSON_WITH_GITHUB: &str = r#"{"author": { "name": "Alice", "github": "example" }}"#;

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
    #[case::toml_structured_email(TOML_WITH_MAIL, str2toml, &EMAIL_AUTHOR)]
    #[case::toml_structured_homepage(TOML_WITH_HOMEPAGE, str2toml, &HOMEPAGE_AUTHOR)]
    #[case::toml_structured_github(TOML_WITH_GITHUB, str2toml, &GITHUB_AUTHOR)]
    #[case::yaml_string(YAML_SIMPLE, str2yaml, &SIMPLE_AUTHOR)]
    #[case::yaml_structured_email(YAML_WITH_MAIL, str2yaml, &EMAIL_AUTHOR)]
    #[case::yaml_structured_homepage(YAML_WITH_HOMEPAGE, str2yaml, &HOMEPAGE_AUTHOR)]
    #[case::yaml_structured_github(YAML_WITH_GITHUB, str2yaml, &GITHUB_AUTHOR)]
    #[case::json_string(JSON_SIMPLE, str2json, &SIMPLE_AUTHOR)]
    #[case::json_structured_email(JSON_WITH_MAIL, str2json, &EMAIL_AUTHOR)]
    #[case::json_structured_homepage(JSON_WITH_HOMEPAGE, str2json, &HOMEPAGE_AUTHOR)]
    #[case::json_structured_github(JSON_WITH_GITHUB, str2json, &GITHUB_AUTHOR)]
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
