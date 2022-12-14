use crate::object::ConfigObject;

#[cfg(feature = "async")]
mod async_source;
mod file;
mod format;
mod string;

#[cfg(feature = "async")]
pub use crate::source::async_source::AsyncConfigSource;
pub use crate::source::file::FileSource;
pub use crate::source::format::FormatParser;
pub use crate::source::format::JsonFormatParser;
pub use crate::source::string::StringSource;

pub trait ConfigSource: std::fmt::Debug {
    fn load(&self) -> Result<ConfigObject, SourceError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "json")]
    #[error("JSON Parser error")]
    JsonParserError(#[from] serde_json::Error),

    #[cfg(feature = "toml")]
    #[error("TOML Parser error")]
    TomlParserError(#[from] toml::de::Error),
}

#[cfg(test)]
pub(crate) mod test_source {
    use crate::description::ConfigSourceDescription;
    use crate::element::ConfigElement;
    use crate::object::ConfigObject;
    use crate::source::ConfigSource;

    use super::SourceError;

    #[derive(Debug)]
    pub(crate) struct TestSource<T: ConfigElement + Clone>(pub(crate) T);

    impl<T> ConfigSource for TestSource<T>
    where
        T: ConfigElement + Clone,
    {
        fn load(&self) -> Result<ConfigObject, SourceError> {
            Ok(ConfigObject::new(
                Box::new(self.0.clone()),
                ConfigSourceDescription::Unknown,
            ))
        }
    }
}
