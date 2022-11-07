use crate::element::IntoConfigElement;

use super::SourceError;

pub trait FormatParser: std::fmt::Debug {
    type Output: IntoConfigElement + std::fmt::Debug + Sized;

    fn parse(buffer: &[u8]) -> Result<Self::Output, SourceError>;
}

#[cfg(feature = "json")]
#[derive(Debug)]
pub struct JsonFormatParser;

#[cfg(feature = "json")]
impl FormatParser for JsonFormatParser {
    type Output = serde_json::Value;

    fn parse(buffer: &[u8]) -> Result<Self::Output, SourceError> {
        serde_json::from_slice(buffer).map_err(SourceError::JsonParserError)
    }
}

#[cfg(feature = "toml")]
#[derive(Debug)]
pub struct TomlFormatParser;

#[cfg(feature = "toml")]
impl FormatParser for TomlFormatParser {
    type Output = toml::Value;

    fn parse(buffer: &[u8]) -> Result<Self::Output, SourceError> {
        toml::from_slice(buffer).map_err(SourceError::TomlParserError)
    }
}
