use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use crate::description::ConfigSourceDescription;
use crate::object::ConfigObject;
use crate::source::format::FormatParser;
use crate::ConfigSource;

use super::SourceError;

#[derive(Debug)]
pub struct FileSource<P>
where
    P: FormatParser + std::fmt::Debug,
{
    path: PathBuf,
    _pd: std::marker::PhantomData<P>,
}

impl<P: FormatParser> FileSource<P> {
    pub fn new<Pa: AsRef<Path>>(source: Pa) -> Result<Self, SourceError> {
        Ok(FileSource {
            path: source.as_ref().to_path_buf(),
            _pd: std::marker::PhantomData,
        })
    }
}

impl<P> ConfigSource for FileSource<P>
where
    P: FormatParser + Debug,
    <P as FormatParser>::Output: 'static,
{
    fn load(&self) -> Result<ConfigObject, SourceError> {
        let buf = std::fs::read(&self.path)?;
        let element = P::parse(buf)?;

        let desc = ConfigSourceDescription::Custom("String".to_string());
        Ok(ConfigObject::new(Box::new(element), desc))
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<P> crate::source::AsyncConfigSource for FileSource<P>
where
    P: FormatParser + Send + Sync + Debug,
    <P as FormatParser>::Output: 'static,
{
    async fn load_async(&self) -> Result<ConfigObject, SourceError> {
        let buf = tokio::fs::read(&self.path).await?;
        let element = P::parse(buf)?;

        let desc = ConfigSourceDescription::Custom("String".to_string());
        Ok(ConfigObject::new(Box::new(element), desc))
    }
}
