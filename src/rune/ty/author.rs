use eyre::Result;
use rune::{Any, ContextError, Module};

#[derive(Debug, PartialEq, Any, Clone)]
#[rune(constructor, item = ::unvoid::author)]
pub struct Author {
    #[rune(get, set)]
    pub name: String,
    #[rune(get, set)]
    pub email: Option<String>,
    #[rune(get, set)]
    pub homepage: Option<String>,
    #[rune(get, set)]
    pub github: Option<String>,
}

impl Author {
    #[rune::function(path = Author::new)]
    fn rune_new(s: String) -> Self {
        Self {
            name: s,
            email: None,
            homepage: None,
            github: None,
        }
    }

    #[rune::function(path = Author::with_email)]
    fn rune_with_email(&self, email: String) -> Self {
        Self {
            email: Some(email),
            ..self.clone()
        }
    }

    #[rune::function(path = Author::with_homepage)]
    fn rune_with_homepage(&self, homepage: String) -> Self {
        Self {
            homepage: Some(homepage),
            ..self.clone()
        }
    }

    #[rune::function(path = Author::with_github)]
    fn rune_with_github(&self, github: String) -> Self {
        Self {
            github: Some(github),
            ..self.clone()
        }
    }
}

#[rune::module(::unvoid::author)]
// pub fn register(_module: &mut Module) -> Result<Module> {
pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::from_meta(self::module_meta)?;
    module.ty::<Author>()?;

    module.function_meta(Author::rune_new)?;
    module.function_meta(Author::rune_with_email)?;
    module.function_meta(Author::rune_with_homepage)?;
    module.function_meta(Author::rune_with_github)?;

    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rune::init_rune_vm;
    use pretty_assertions::assert_eq;
    use rune::sources;

    #[test]
    fn test_author_only_name() -> Result<()> {
        let mut sources = sources! {
            author => {
                use ::unvoid::author::Author;

                pub fn main() {
                    Author::new("John Doe")
                }
            }
        };

        let author: Author = rune::from_value(init_rune_vm(&mut sources)?.call(["main"], ())?)?;

        assert_eq!(
            author,
            Author {
                name: "John Doe".into(),
                email: None,
                homepage: None,
                github: None,
            }
        );

        Ok(())
    }

    #[test]
    fn test_author_with_email() -> Result<()> {
        let mut sources = sources! {
            author => {
                use ::unvoid::author::Author;

                pub fn main() {
                    Author::new("John Doe").with_email("jon.doe@example.com")
                }
            }
        };

        let author: Author = rune::from_value(init_rune_vm(&mut sources)?.call(["main"], ())?)?;

        assert_eq!(
            author,
            Author {
                name: "John Doe".into(),
                email: Some("jon.doe@example.com".into()),
                homepage: None,
                github: None,
            }
        );

        Ok(())
    }

    #[test]
    fn test_author_with_homepage() -> Result<()> {
        let mut sources = sources! {
            author => {
                use ::unvoid::author::Author;

                pub fn main() {
                    Author::new("John Doe").with_homepage("https://example.com")
                }
            }
        };

        let author: Author = rune::from_value(init_rune_vm(&mut sources)?.call(["main"], ())?)?;

        assert_eq!(
            author,
            Author {
                name: "John Doe".into(),
                email: None,
                homepage: Some("https://example.com".into()),
                github: None,
            }
        );

        Ok(())
    }

    #[test]
    fn test_author_with_github() -> Result<()> {
        let mut sources = sources! {
            author => {
                use ::unvoid::author::Author;

                pub fn main() {
                    Author::new("John Doe").with_github("example")
                }
            }
        };

        let author: Author = rune::from_value(init_rune_vm(&mut sources)?.call(["main"], ())?)?;

        assert_eq!(
            author,
            Author {
                name: "John Doe".into(),
                email: None,
                homepage: None,
                github: Some("example".into()),
            }
        );

        Ok(())
    }

    #[test]
    fn test_author_with_everything() -> Result<()> {
        let mut sources = sources! {
            author => {
                use ::unvoid::author::Author;

                pub fn main() {
                    Author::new("John Doe")
                        .with_email("jon.doe@example.com")
                        .with_homepage("https://example.com")
                        .with_github("example")
                }
            }
        };

        let author: Author = rune::from_value(init_rune_vm(&mut sources)?.call(["main"], ())?)?;

        assert_eq!(
            author,
            Author {
                name: "John Doe".into(),
                email: Some("jon.doe@example.com".into()),
                homepage: Some("https://example.com".into()),
                github: Some("example".into()),
            }
        );

        Ok(())
    }
}
