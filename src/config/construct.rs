use super::Layers;

pub trait ConfigConstructor
where
    Self: Sized,
{
    type Error;

    fn construct_from(layers: &Layers) -> Result<Self, Self::Error>;
}

#[cfg(all(test, feature = "toml"))]
mod tests {
    use crate::source::TomlFormatParser;
    use crate::{config::ConfigError, Config, StringSource};

    use super::*;

    #[derive(Debug)]
    struct SimpleConfig {
        s: String,
    }

    impl ConfigConstructor for SimpleConfig {
        type Error = ConfigError;

        fn construct_from(layers: &Layers) -> Result<Self, Self::Error> {
            let s = layers
                .get("s")?
                .expect("No 's' in layers found")
                .as_str()
                .expect("'s' is not a string")
                .to_string();

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
}
