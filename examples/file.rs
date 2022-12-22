#[cfg(not(feature = "async"))]
fn run_example() {
    use config_rs_ng::Config;
    use config_rs_ng::FileSource;
    use config_rs_ng::JsonFormatParser;

    let config_file = std::env::current_dir()
        .expect("Finding the current directory")
        .join("examples")
        .join("file.json");

    println!("Loading file: {}", config_file.display());

    let config = Config::builder()
        .load(Box::new({
            FileSource::<JsonFormatParser>::new(config_file).expect("building FileSource")
        }))
        .build()
        .expect("Building configuration object");

    let key = config
        .get("foo")
        .expect("Accessing configuration object")
        .expect("Finding 'key' in configuration object");

    println!("'key' Config element is: '{:?}'", key);
}

#[cfg(feature = "async")]
fn run_example() {}

fn main() {
    run_example()
}
