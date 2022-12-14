use crate::accessor::Accessor;
use crate::description::ConfigSourceDescription;
use crate::element::ConfigElement;

#[derive(Debug)]
pub struct ConfigObject {
    element: Box<dyn ConfigElement>,
    #[allow(unused)] // TODO
    source: ConfigSourceDescription,
}

impl ConfigObject {
    pub fn new(element: Box<dyn ConfigElement>, source: ConfigSourceDescription) -> Self {
        Self { element, source }
    }

    pub(crate) fn get(
        &self,
        accessor: &mut Accessor,
    ) -> Result<Option<&dyn ConfigElement>, ConfigObjectAccessError> {
        self.element.access(accessor)
    }

    pub(crate) fn get_as<T: ConfigElement>(
        &self,
        accessor: &mut Accessor,
    ) -> Result<Option<&T>, ConfigObjectAccessError> {
        Ok(self.get(accessor)?.and_then(|t| t.downcast_ref()))
    }

    pub(crate) fn get_with_description<'a>(
        &'a self,
        accessor: &mut Accessor,
    ) -> Result<Option<ConfigView<'a>>, ConfigObjectAccessError> {
        if let Some(element) = self.get(accessor)? {
            Ok(Some({
                ConfigView {
                    element,
                    desc: &self.source,
                }
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigObjectAccessError {
    #[error("Tried to access with no accessor")]
    NoAccessor,

    #[error("Accessed Null with key '{0}'")]
    AccessWithKeyOnNull(String),
    #[error("Accessed Bool with key '{0}'")]
    AccessWithKeyOnBool(String),
    #[error("Accessed i8 with key '{0}'")]
    AccessWithKeyOnI8(String),
    #[error("Accessed i16 with key '{0}'")]
    AccessWithKeyOnI16(String),
    #[error("Accessed i32 with key '{0}'")]
    AccessWithKeyOnI32(String),
    #[error("Accessed i64 with key '{0}'")]
    AccessWithKeyOnI64(String),
    #[error("Accessed u8 with key '{0}'")]
    AccessWithKeyOnU8(String),
    #[error("Accessed u16 with key '{0}'")]
    AccessWithKeyOnU16(String),
    #[error("Accessed u32 with key '{0}'")]
    AccessWithKeyOnU32(String),
    #[error("Accessed u64 with key '{0}'")]
    AccessWithKeyOnU64(String),
    #[error("Accessed f32 with key '{0}'")]
    AccessWithKeyOnF32(String),
    #[error("Accessed f64 with key '{0}'")]
    AccessWithKeyOnF64(String),
    #[error("Accessed String with key '{0}'")]
    AccessWithKeyOnStr(String),
    #[error("Accessed List with key '{0}'")]
    AccessWithKeyOnList(String),

    #[error("Accessed Null with index '{0}'")]
    AccessWithIndexOnNull(usize),
    #[error("Accessed Bool with index '{0}'")]
    AccessWithIndexOnBool(usize),
    #[error("Accessed i8 with index '{0}'")]
    AccessWithIndexOnI8(usize),
    #[error("Accessed i16 with index '{0}'")]
    AccessWithIndexOnI16(usize),
    #[error("Accessed i32 with index '{0}'")]
    AccessWithIndexOnI32(usize),
    #[error("Accessed i64 with index '{0}'")]
    AccessWithIndexOnI64(usize),
    #[error("Accessed u8 with index '{0}'")]
    AccessWithIndexOnU8(usize),
    #[error("Accessed u16 with index '{0}'")]
    AccessWithIndexOnU16(usize),
    #[error("Accessed u32 with index '{0}'")]
    AccessWithIndexOnU32(usize),
    #[error("Accessed u64 with index '{0}'")]
    AccessWithIndexOnU64(usize),
    #[error("Accessed f32 with index '{0}'")]
    AccessWithIndexOnF32(usize),
    #[error("Accessed f64 with index '{0}'")]
    AccessWithIndexOnF64(usize),
    #[error("Accessed usize with index '{0}'")]
    AccessWithIndexOnStr(usize),
    #[error("Accessed Map with index '{0}'")]
    AccessWithIndexOnMap(usize),
}

/// An object that can be used to get a configuration value or the description of the source of
/// that value.
#[derive(Debug)]
pub struct ConfigView<'a> {
    element: &'a dyn ConfigElement,
    desc: &'a ConfigSourceDescription,
}

impl<'a> ConfigView<'a> {
    pub fn value(&self) -> &dyn ConfigElement {
        self.element
    }

    pub fn description(&self) -> &ConfigSourceDescription {
        self.desc
    }
}
