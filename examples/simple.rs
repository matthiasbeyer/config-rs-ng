#[cfg(not(feature = "async"))]
fn run_example() {
    use config_rs_ng::Config;
    use config_rs_ng::JsonFormatParser;
    use config_rs_ng::StringSource;

    const CONFIGURATION: &str = r#"
    {
        "key": "value"
    }
    "#;

    let config = Config::builder()
        .load(Box::new({
            StringSource::<JsonFormatParser>::new(CONFIGURATION.to_string())
                .expect("building StringSource")
        }))
        .build()
        .expect("Building configuration object");

    let key = config
        .get("key")
        .expect("Accessing configuration object")
        .expect("Finding 'key' in configuration object");

    println!("'key' Config element is: '{:?}'", key);
}

#[cfg(feature = "async")]
fn run_example() {}

fn main() {
    run_example()
}
