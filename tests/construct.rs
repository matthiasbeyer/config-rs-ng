#![cfg(feature = "toml")]

use config_rs_ng::Config;
use config_rs_ng::ConfigConstructor;
use config_rs_ng::FromConfigElement;
use config_rs_ng::Layers;
use config_rs_ng::StringSource;
use config_rs_ng::TomlFormatParser;

#[derive(Debug)]
struct SimpleConfig {
    s: String,
}

type ConfigError = ();

impl ConfigConstructor for SimpleConfig {
    type Error = ConfigError;

    fn construct_from(layers: &Layers) -> Result<Self, Self::Error> {
        let element = layers
            .get("s")
            .map_err(|_| ())? // Dont care about that here in test code
            .expect("No 's' in layers found");

        let s = String::from_config_element(element).expect("string");
        Ok(SimpleConfig { s })
    }
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
