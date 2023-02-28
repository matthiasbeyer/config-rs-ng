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

#[derive(Debug, config_rs_ng::ConfigConstructor)]
struct OtherConfig {
    optional: Option<i32>,

    simple: SimpleConfig,
}

impl config_rs_ng::FromConfigElement for SimpleConfig {
    type Error = config_rs_ng::FromConfigElementError;

    fn from_config_element(element: &dyn config_rs_ng::ConfigElement) -> Result<Self, Self::Error> {
        let map = element.as_map().ok_or_else(|| {
            let found = element.get_type().name();
            config_rs_ng::FromConfigElementError::TypeError {
                expected: "map",
                found,
            }
        })?;

        Ok({
            Self {
                s: map
                    .get("s")
                    .ok_or_else(|| config_rs_ng::FromConfigElementError::NoElement {
                        name: "s".to_string(),
                        ty: "String".to_string(),
                    })
                    .and_then(String::from_config_element)?
            }
        })
    }
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
