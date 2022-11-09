use config_rs_ng::Config;
use config_rs_ng::JsonFormatParser;
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

#[test]
fn test_layers_simple() {
    let config = Config::builder()
        .load(Box::new({
            StringSource::<JsonFormatParser>::new(CONFIGURATION_LAYER_A.to_string())
                .expect("building StringSource")
        }))
        .load(Box::new({
            StringSource::<JsonFormatParser>::new(CONFIGURATION_LAYER_B.to_string())
                .expect("building StringSource")
        }))
        .build()
        .expect("Building configuration object");

    {
        let value = config
            .get("key1")
            .expect("Accessing configuration object")
            .expect("Finding 'key' in configuration object");

        assert!(value.is_str());
        let s = value.as_str().unwrap();
        assert_eq!(s, "valueA");
    }
    {
        let value = config
            .get("key2")
            .expect("Accessing configuration object")
            .expect("Finding 'key' in configuration object");

        assert!(value.is_str());
        let s = value.as_str().unwrap();
        assert_eq!(s, "valueB");
    }
    {
        let value = config
            .get("key3")
            .expect("Accessing configuration object")
            .expect("Finding 'key' in configuration object");

        assert!(value.is_str());
        let s = value.as_str().unwrap();
        assert_eq!(s, "valueB");
    }
}
