//! Asynchronous configuration source

use crate::object::ConfigObject;
use crate::source::SourceError;

#[async_trait::async_trait]
pub trait AsyncConfigSource: std::fmt::Debug {
    async fn load_async(&self) -> Result<ConfigObject, SourceError>;
}
