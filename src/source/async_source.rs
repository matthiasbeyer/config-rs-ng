//! Asynchronous configuration source

use crate::object::ConfigObject;
use crate::source::SourceError;

/// A source of a configuration that can be loaded asyncronously
///
/// # Note
///
/// See [ConfigSource](crate::ConfigSource) for an sync variant of this trait.
///
#[async_trait::async_trait]
pub trait AsyncConfigSource: std::fmt::Debug {
    async fn load_async(&self) -> Result<ConfigObject, SourceError>;
}
