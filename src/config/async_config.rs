use crate::config::AsyncConfigBuilder;
use crate::config::ConfigError;
use crate::config::Layers;

#[derive(Debug)]
pub struct AsyncConfig {
    builder: AsyncConfigBuilder,
    layers: crate::config::layers::Layers,
}

impl AsyncConfig {
    pub fn builder() -> AsyncConfigBuilder {
        AsyncConfigBuilder::new()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub(super) async fn build_from_builder(
        builder: crate::config::AsyncConfigBuilder,
    ) -> Result<Self, ConfigError> {
        let layers: crate::config::layers::Layers = builder.reload().await?;
        Ok(Self { layers, builder })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub async fn reload(&mut self) -> Result<(), ConfigError> {
        let layers: crate::config::layers::Layers = self.builder.reload().await?;
        self.layers = layers;
        Ok(())
    }

    /// Get the configuration layers and via them access to the actual values.
    pub fn layers(&self) -> &Layers {
        &self.layers
    }
}
