#[cfg(feature = "async")]
mod async_builder;
#[cfg(not(feature = "async"))]
mod builder;
#[allow(clippy::module_inception)]
mod config;
mod error;

#[cfg(feature = "async")]
pub use crate::config::async_builder::*;
#[cfg(not(feature = "async"))]
pub use crate::config::builder::*;
pub use crate::config::config::*;
pub use crate::config::error::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_loading() {
        let _c = Config::builder()
            .load(Box::new(crate::source::test_source::TestSource(
                toml::Value::String("".to_string()),
            )))
            .build()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json() {
        let json: serde_json::Value = serde_json::from_str(
            r#"
            { "key": "value" }
        "#,
        )
        .unwrap();

        let _c = Config::builder()
            .load(Box::new(crate::source::test_source::TestSource(json)))
            .build()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_load_json_get_value() {
        let json: serde_json::Value = serde_json::from_str(
            r#"
            { "key": "value" }
        "#,
        )
        .unwrap();

        let source = crate::source::test_source::TestSource(json);

        let c = Config::builder().load(Box::new(source)).build().unwrap();

        let r = c.get("key");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "value");
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_layered_json_config() {
        let json1: serde_json::Value = serde_json::from_str(
            r#"
            { "key1": "value1" }
        "#,
        )
        .unwrap();

        let json2: serde_json::Value = serde_json::from_str(
            r#"
            { "key1": "value2", "key2": "value3" }
        "#,
        )
        .unwrap();

        let source1 = crate::source::test_source::TestSource(json1);
        let source2 = crate::source::test_source::TestSource(json2);

        let c = Config::builder()
            .load(Box::new(source1))
            .load(Box::new(source2))
            .build()
            .unwrap();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "value2");

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "value3");
    }

    #[test]
    #[cfg(all(feature = "json", feature = "toml"))]
    fn test_layered_json_toml_config() {
        let json: serde_json::Value = serde_json::from_str(
            r#"
            { "key1": "value1" }
        "#,
        )
        .unwrap();

        let toml: toml::Value = toml::from_str(
            r#"
            key1 = "value2"
            key2 = "value3"
        "#,
        )
        .unwrap();

        let source1 = crate::source::test_source::TestSource(json);
        let source2 = crate::source::test_source::TestSource(toml);

        let c = Config::builder()
            .load(Box::new(source1))
            .load(Box::new(source2))
            .build()
            .unwrap();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "value2");

        let r = c.get("key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "value3");
    }
}
