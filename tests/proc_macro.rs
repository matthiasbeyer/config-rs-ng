#![cfg(feature = "toml")]

use config_rs_ng::Config;
use config_rs_ng::ConfigConstructor;
use config_rs_ng::StringSource;
use config_rs_ng::TomlFormatParser;

#[derive(Debug, config_rs_ng::ConfigConstructor, config_rs_ng::FromConfigElement)]
struct SimpleConfig {
    s: String,
}

const CONFIG: &str = r#"
    s = "foo"
"#;

#[test]
fn test() {
    let config = Config::builder()
        .load(Box::new(
            StringSource::<TomlFormatParser>::new(CONFIG.to_string()).unwrap(),
        ))
        .build()
        .unwrap();

    let simple = SimpleConfig::construct_from(config.layers()).unwrap();
    assert_eq!(simple.s, "foo");
}

#[derive(Debug, config_rs_ng::ConfigConstructor)]
struct OtherConfig {
    optional: Option<i32>,

    simple: SimpleConfig,
}

const OTHER_CONFIG: &str = r#"
    optional = 12
    [simple]
    s = "bar"
"#;

#[test]
fn test_config() {
    let config = Config::builder()
        .load(Box::new(
            StringSource::<TomlFormatParser>::new(OTHER_CONFIG.to_string()).unwrap(),
        ))
        .build()
        .unwrap();

    let other = OtherConfig::construct_from(config.layers()).unwrap();
    assert_eq!(other.optional, Some(12));
    assert_eq!(other.simple.s, "bar");
}
