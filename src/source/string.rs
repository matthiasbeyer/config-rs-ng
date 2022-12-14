use crate::description::ConfigSourceDescription;
use crate::object::ConfigObject;
use crate::source::format::FormatParser;
use crate::ConfigSource;

use super::SourceError;

#[derive(Debug)]
pub struct StringSource<P: FormatParser + std::fmt::Debug> {
    source: String,
    _pd: std::marker::PhantomData<P>,
}

impl<P: FormatParser> StringSource<P> {
    pub fn new(source: String) -> Result<Self, SourceError> {
        Ok(StringSource {
            source,
            _pd: std::marker::PhantomData,
        })
    }
}

impl<P> ConfigSource for StringSource<P>
where
    P: FormatParser + std::fmt::Debug,
    <P as FormatParser>::Output: 'static,
{
    fn load(&self) -> Result<ConfigObject, SourceError> {
        let element = P::parse(self.source.as_bytes().to_vec())?;

        let desc = ConfigSourceDescription::Custom("String".to_string());
        Ok(ConfigObject::new(Box::new(element), desc))
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<P> crate::source::AsyncConfigSource for StringSource<P>
where
    P: FormatParser + std::marker::Sync + std::fmt::Debug,
    <P as FormatParser>::Output: 'static,
{
    async fn load_async(&self) -> Result<ConfigObject, SourceError> {
        let element = P::parse(self.source.as_bytes().to_vec())?;

        let desc = ConfigSourceDescription::Custom("String".to_string());
        Ok(ConfigObject::new(Box::new(element), desc))
    }
}

#[cfg(test)]
mod test_source_impl {
    #[cfg(feature = "json")]
    #[test]
    fn test_json_string_source() {
        use super::*;

        let source = "{}";

        let source =
            StringSource::<crate::source::JsonFormatParser>::new(source.to_string()).unwrap();
        let _object = source.load().unwrap();
    }
}
