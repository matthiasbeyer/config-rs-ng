#[cfg(not(feature = "async"))]
async fn run_example() {}

#[cfg(feature = "async")]
async fn run_example() {
    use config_rs_ng::AsyncConfig;
    use config_rs_ng::FileSource;
    use config_rs_ng::JsonFormatParser;

    let config_file = std::env::current_dir()
        .expect("Finding the current directory")
        .join("examples")
        .join("file.json");

    println!("Loading file: {}", config_file.display());

    let config = AsyncConfig::builder()
        .load(Box::new({
            FileSource::<JsonFormatParser>::new(config_file).expect("building FileSource")
        }))
        .build()
        .await
        .expect("Building configuration object");

    let key = config
        .layers()
        .get("foo")
        .expect("Accessing configuration object")
        .expect("Finding 'key' in configuration object");

    println!("'key' Config element is: '{:?}'", key);
}

#[tokio::main]
async fn main() {
    run_example().await
}
