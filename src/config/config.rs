use crate::accessor::Accessor;
use crate::accessor::ParsableAccessor;
use crate::config::ConfigBuilder;
use crate::config::ConfigError;
use crate::element::ConfigElement;
use crate::object::ConfigObject;

#[derive(Debug)]
pub struct Config {
    layers: Vec<ConfigObject>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    #[cfg(feature = "async")]
    pub fn async_builder() -> crate::config::AsyncConfigBuilder {
        crate::config::AsyncConfigBuilder::new()
    }

    pub(super) fn build_from_builder(builder: &ConfigBuilder) -> Result<Self, ConfigError> {
        let config = Config {
            layers: builder.reload()?,
        };

        Ok(config)
    }

    #[cfg(feature = "async")]
    pub(super) async fn build_from_async_builder(
        builder: &crate::config::AsyncConfigBuilder,
    ) -> Result<Self, ConfigError> {
        let config = Config {
            layers: builder.reload().await?,
        };

        Ok(config)
    }

    /// Access the configuration at a specific position
    ///
    /// Use an object of a type implementing the `ParsableAccessor` trait for accessing the
    /// configuration at a certain position.
    /// As `ParsableAccessor` is implemented by [`&str`] and [`String`], passing those directly
    /// works.
    ///
    /// # Note
    ///
    /// Each time, [`Config::get`] is called, the `ParsableAccessor::parse()` function is called.
    /// If that is a unbearable overhead (especially in cases where the accessor is hard-coded),
    /// [`Config::get_with_accessor`] can be used to prevent that overhead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use config_rs_ng::Config;
    /// let config: Config = { //...
    /// # unimplemented!()
    /// };
    ///
    /// config.get("foo")
    ///     // ...
    /// # ;
    /// ```
    pub fn get<A>(&self, accessor: A) -> Result<Option<&dyn ConfigElement>, ConfigError>
    where
        A: ParsableAccessor,
    {
        let accessor = accessor.parse()?;
        self.get_with_accessor(accessor)
    }

    /// Access the configuration at a specific position
    ///
    /// See [`Config::get`]
    pub fn get_with_accessor(
        &self,
        mut accessor: Accessor,
    ) -> Result<Option<&dyn ConfigElement>, ConfigError> {
        for layer in self.layers.iter().rev() {
            if let Some(value) = layer.get(&mut accessor)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }

    pub fn get_as<A, T>(&self, accessor: A) -> Result<Option<&T>, ConfigError>
    where
        A: ParsableAccessor,
        T: ConfigElement,
    {
        let accessor = accessor.parse()?;
        self.get_with_accessor_as::<T>(accessor)
    }

    pub fn get_with_accessor_as<T>(&self, mut accessor: Accessor) -> Result<Option<&T>, ConfigError>
    where
        T: ConfigElement,
    {
        for layer in self.layers.iter().rev() {
            if let Some(value) = layer.get_as::<T>(&mut accessor)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}
