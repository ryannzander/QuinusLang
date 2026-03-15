use quinuslang::package;
use std::path::Path;

#[test]
fn test_parse_manifest() {
    let manifest_path = Path::new("quinus.toml");
    if manifest_path.exists() {
        let manifest = package::manifest::parse_manifest(manifest_path).unwrap();
        assert_eq!(manifest.package.name, "my-app");
        assert_eq!(manifest.build.entry, "src/main.q");
    }
}
