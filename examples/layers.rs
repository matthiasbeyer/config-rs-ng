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

fn main() {
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

    for key in ["key1", "key2", "key3"] {
        let value = config
            .get(key)
            .expect("Accessing configuration object")
            .expect("Finding 'key' in configuration object");

        println!("'{}' Config element is: '{:?}'", key, value);
    }
}
