use crate::config::ConfigBuilder;
use crate::config::ConfigError;
use crate::config::Layers;

#[derive(Debug)]
pub struct Config {
    builder: ConfigBuilder,
    layers: crate::config::layers::Layers,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub(super) fn build_from_builder(builder: ConfigBuilder) -> Result<Self, ConfigError> {
        Ok(Config {
            layers: builder.reload()?,
            builder,
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn reload(&mut self) -> Result<(), ConfigError> {
        let layers = self.builder.reload()?;
        self.layers = layers;
        Ok(())
    }

    /// Get the configuration layers and via them access to the actual values.
    pub fn layers(&self) -> &Layers {
        &self.layers
    }
}
