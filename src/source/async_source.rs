//! Asynchronous configuration source

use crate::object::ConfigObject;
use crate::source::SourceError;

#[async_trait::async_trait]
pub trait AsyncConfigSource {
    async fn load_async(&self) -> Result<ConfigObject, SourceError>;
}
