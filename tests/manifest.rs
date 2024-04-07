use rune::Source;

use unvoid::manifest::Manifest;
use unvoid::version::Version;

#[test]
fn minimal_seserializes_correctly_from_rune_file() {
    let source = Source::from_path("tests/manifests/simple.rn").unwrap();

    let manifest = dbg!(Manifest::from_rune(source)).unwrap();

    assert_eq!(manifest.name, "my-project");
    assert_eq!(manifest.version, Version::new(0, 1, 0));
    assert_eq!(
        manifest.authors,
        vec!["Alice".to_string(), "Bob".to_string()]
    );
}