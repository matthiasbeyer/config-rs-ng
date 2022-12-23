#![cfg(not(feature = "async"))]
use std::sync::Mutex;

use config_rs_ng::Config;
use config_rs_ng::ConfigObject;
use config_rs_ng::ConfigSource;
use config_rs_ng::ConfigSourceDescription;
use config_rs_ng::JsonFormatParser;
use config_rs_ng::SourceError;
use config_rs_ng::StringSource;

const CONFIGURATION_LAYER_A: &str = r#"
{
    "key1": "valueA",
    "key2": "valueA"
}
"#;

const CONFIGURATION_LAYER_B: &str = r#"
{
    "key2": "valueB",
    "key3": "valueB"
}
"#;

const CONFIGURATION_LAYER_C: &str = r#"
{
    "key4": "valueC"
}
"#;

// A helper type as custom configuration source, that, when reloaded, returns another
// configuration.
#[derive(Debug)]
pub struct CustomSource(Mutex<bool>);

impl ConfigSource for CustomSource {
    fn load(&self) -> Result<ConfigObject, SourceError> {
        fn load_src(src: &str) -> ConfigObject {
            let conf = serde_json::from_str::<serde_json::Value>(src).unwrap();
            let desc = ConfigSourceDescription::Unknown;
            ConfigObject::new(Box::new(conf), desc)
        }

        let mut b = self.0.lock().unwrap();
        if *b {
            *b = false;
            Ok(load_src(CONFIGURATION_LAYER_B))
        } else {
            Ok(load_src(CONFIGURATION_LAYER_C))
        }
    }
}

#[test]
fn test_reloading() {
    let mut config = Config::builder()
        .load(Box::new({
            StringSource::<JsonFormatParser>::new(CONFIGURATION_LAYER_A.to_string())
                .expect("building StringSource")
        }))
        .load(Box::new({ CustomSource(Mutex::new(true)) }))
        .build()
        .expect("Building configuration object");

    fn assert_key_val(config: &Config, key: &str, val: &str) {
        let value = config
            .layers()
            .get(key)
            .expect("Accessing configuration object")
            .expect(&format!("Finding '{}' in configuration object", key));

        assert!(value.is_str());
        let s = value.as_str().unwrap();
        assert_eq!(s, val);
    }

    assert_key_val(&config, "key1", "valueA");
    assert_key_val(&config, "key2", "valueB");
    assert_key_val(&config, "key3", "valueB");
    assert!({
        config
            .layers()
            .get("key4")
            .expect("Accessing configuration object")
            .is_none()
    });

    config.reload().unwrap();

    assert_key_val(&config, "key1", "valueA");
    assert_key_val(&config, "key2", "valueB");
    assert_key_val(&config, "key4", "valueC");
    assert!({
        config
            .layers()
            .get("key3")
            .expect("Accessing configuration object")
            .is_none()
    });
}
