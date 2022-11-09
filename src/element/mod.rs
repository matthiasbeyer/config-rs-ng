use std::collections::HashMap;

use crate::{
    accessor::{AccessType, Accessor},
    object::ConfigObjectAccessError,
};

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum ConfigElement {
    Null,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Str(String),
    List(Vec<ConfigElement>),
    Map(HashMap<String, ConfigElement>),
}

impl ConfigElement {
    pub(crate) fn get(
        &self,
        accessor: &mut Accessor,
    ) -> Result<Option<&ConfigElement>, ConfigObjectAccessError> {
        match (accessor.current(), &self) {
            (Some(AccessType::Key(k)), ConfigElement::Null) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnNull(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::Bool(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnBool(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I8(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI8(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I16(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI16(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I32(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI32(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::I64(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI64(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U8(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU8(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U16(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU16(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U32(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU32(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::U64(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU64(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::F32(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnF32(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::F64(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnF64(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::Str(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnStr(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::List(_)) => {
                Err(ConfigObjectAccessError::AccessWithKeyOnList(k.to_string()))
            }
            (Some(AccessType::Key(k)), ConfigElement::Map(hm)) => {
                if let Some(value) = hm.get(k.as_str()) {
                    accessor.advance();
                    if accessor.current().is_none() {
                        Ok(Some(value))
                    } else {
                        value.get(accessor)
                    }
                } else {
                    Ok(None)
                }
            }

            (Some(AccessType::Index(u)), ConfigElement::Null) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnNull(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::Bool(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnBool(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I8(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI8(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I16(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI16(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I32(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI32(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::I64(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI64(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U8(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU8(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U16(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU16(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U32(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU32(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::U64(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU64(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::F32(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnF32(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::F64(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnF64(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::Str(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnStr(*u))
            }
            (Some(AccessType::Index(u)), ConfigElement::List(v)) => {
                if let Some(value) = v.get(*u) {
                    accessor.advance();
                    if accessor.current().is_none() {
                        Ok(Some(value))
                    } else {
                        value.get(accessor)
                    }
                } else {
                    Ok(None)
                }
            }
            (Some(AccessType::Index(u)), ConfigElement::Map(_)) => {
                Err(ConfigObjectAccessError::AccessWithIndexOnMap(*u))
            }

            (None, _) => Err(ConfigObjectAccessError::NoAccessor),
        }
    }

    pub fn is_null(&self) -> bool {
        std::matches!(self, ConfigElement::Null)
    }

    pub fn is_bool(&self) -> bool {
        std::matches!(self, ConfigElement::Bool(_))
    }

    pub fn is_i8(&self) -> bool {
        std::matches!(self, ConfigElement::I8(_))
    }

    pub fn is_i16(&self) -> bool {
        std::matches!(self, ConfigElement::I16(_))
    }

    pub fn is_i32(&self) -> bool {
        std::matches!(self, ConfigElement::I32(_))
    }

    pub fn is_i64(&self) -> bool {
        std::matches!(self, ConfigElement::I64(_))
    }

    pub fn is_u8(&self) -> bool {
        std::matches!(self, ConfigElement::U8(_))
    }

    pub fn is_u16(&self) -> bool {
        std::matches!(self, ConfigElement::U16(_))
    }

    pub fn is_u32(&self) -> bool {
        std::matches!(self, ConfigElement::U32(_))
    }

    pub fn is_u64(&self) -> bool {
        std::matches!(self, ConfigElement::U64(_))
    }

    pub fn is_str(&self) -> bool {
        std::matches!(self, ConfigElement::Str(_))
    }

    pub fn is_list(&self) -> bool {
        std::matches!(self, ConfigElement::List(_))
    }

    pub fn is_map(&self) -> bool {
        std::matches!(self, ConfigElement::Map(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let ConfigElement::Bool(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        if let ConfigElement::I8(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        if let ConfigElement::I16(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        if let ConfigElement::I32(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let ConfigElement::I64(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        if let ConfigElement::U8(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        if let ConfigElement::U16(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        if let ConfigElement::U32(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        if let ConfigElement::U64(o) = self {
            Some(*o)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let ConfigElement::Str(ref o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&[ConfigElement]> {
        if let ConfigElement::List(ref o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, ConfigElement>> {
        if let ConfigElement::Map(ref o) = self {
            Some(o)
        } else {
            None
        }
    }
}

pub trait IntoConfigElement {
    type Error: std::error::Error;

    fn into_config_element(self) -> Result<ConfigElement, Self::Error>;
}

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "toml")]
pub mod toml;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "toml")]
    fn test_nested_toml_config() {
        use crate::element::ConfigElement;
        use crate::element::IntoConfigElement;
        use crate::Config;

        let toml: toml::Value = toml::from_str(
            r#"
            key1 = "value2"

            [table]
            key2 = "value3"
        "#,
        )
        .unwrap();

        let source = crate::source::test_source::TestSource(toml.into_config_element().unwrap());

        let c = Config::builder().load(Box::new(source)).build().unwrap();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "value2"),
            _ => panic!(),
        }

        let r = c.get("table.key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "value3"),
            _ => panic!(),
        }
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_nested_toml_config_with_index() {
        use crate::element::ConfigElement;
        use crate::element::IntoConfigElement;
        use crate::Config;

        let toml: toml::Value = toml::from_str(
            r#"
            [[key]]
            k = "a"

            [[key]]
            k = "b"
        "#,
        )
        .unwrap();

        let source = crate::source::test_source::TestSource(toml.into_config_element().unwrap());

        let c = Config::builder().load(Box::new(source)).build().unwrap();

        let r = c.get("key.0.k");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        match r {
            ConfigElement::Str(s) => assert_eq!(s, "a"),
            _ => panic!(),
        }
    }
}
