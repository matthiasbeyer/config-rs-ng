#![cfg(feature = "toml")]

use config_rs_ng::Config;
use config_rs_ng::ConfigConstructor;
use config_rs_ng::StringSource;
use config_rs_ng::TomlFormatParser;

#[derive(Debug, config_rs_ng::ConfigConstructor)]
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
