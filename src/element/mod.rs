use crate::{
    accessor::{AccessType, Accessor},
    object::ConfigObjectAccessError,
};

pub trait ConfigElementListType {
    fn len(&self) -> usize;

    // This function strangely has to be named something else than `get` because we cannot use
    // `Vec::get` in the impl for Vec<T> otherwise.
    fn at_index(&self, index: usize) -> Option<&dyn ConfigElement>;
}

impl<T> ConfigElementListType for Vec<T>
where
    T: ConfigElement,
{
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn at_index(&self, index: usize) -> Option<&dyn ConfigElement> {
        self.get(index).map(|t| t as &dyn ConfigElement)
    }
}

pub trait ConfigElementMapType {
    fn get(&self, key: &str) -> Option<&dyn ConfigElement>;

    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    fn keys(&self) -> Vec<String>;
    fn values(&self) -> Vec<&dyn ConfigElement>;
}

/// The ConfigElement trait that makes a type usable with this crate
///
/// # Warning
///
/// Each implementation of this trait MUST guarantee that one of the functions named `as_*` return
/// `Some(_)`.
/// In other words: If there can be an object constructed where all `as_*` functions return `None`,
/// all guarantees of this library are void and panics might happen in this library.
///
/// # Implementing
///
/// A user of this library may only implement the `as_*` functions of this trait. The `is_*`
/// functions are auto-implemented by calling the corrosponding `as_*` function and checking the
/// returned `Option` with `Option::is_some()`. If it is the case that that behaviour is
/// inefficient for a implementation of this trait, the user is welcome to implement the `is_*`
/// functions themselves!
///
/// The `get_type()` function is auto-implemented using the `is_*` functions, to return a
/// `ConfigElementType`, that is internally used in this library.
///
/// The `access()` function should not be overridden by the user. It is used by this library to
/// traverse the object tree when accessing values in a configuration.
pub trait ConfigElement: std::fmt::Debug + downcast_rs::DowncastSync {
    fn as_bool(&self) -> Option<bool>;
    fn as_i8(&self) -> Option<i8>;
    fn as_i16(&self) -> Option<i16>;
    fn as_i32(&self) -> Option<i32>;
    fn as_i64(&self) -> Option<i64>;
    fn as_u8(&self) -> Option<u8>;
    fn as_u16(&self) -> Option<u16>;
    fn as_u32(&self) -> Option<u32>;
    fn as_u64(&self) -> Option<u64>;
    fn as_f32(&self) -> Option<f32>;
    fn as_f64(&self) -> Option<f64>;
    fn as_str(&self) -> Option<&str>;
    fn as_list(&self) -> Option<&dyn ConfigElementListType>;
    fn as_map(&self) -> Option<&dyn ConfigElementMapType>;

    fn is_null(&self) -> bool;
    fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }
    fn is_i8(&self) -> bool {
        self.as_i8().is_some()
    }
    fn is_i16(&self) -> bool {
        self.as_i16().is_some()
    }
    fn is_i32(&self) -> bool {
        self.as_i32().is_some()
    }
    fn is_i64(&self) -> bool {
        self.as_i64().is_some()
    }
    fn is_u8(&self) -> bool {
        self.as_u8().is_some()
    }
    fn is_u16(&self) -> bool {
        self.as_u16().is_some()
    }
    fn is_u32(&self) -> bool {
        self.as_u32().is_some()
    }
    fn is_u64(&self) -> bool {
        self.as_u64().is_some()
    }
    fn is_f32(&self) -> bool {
        self.as_f32().is_some()
    }
    fn is_f64(&self) -> bool {
        self.as_f64().is_some()
    }
    fn is_str(&self) -> bool {
        self.as_str().is_some()
    }
    fn is_list(&self) -> bool {
        self.as_list().is_some()
    }
    fn is_map(&self) -> bool {
        self.as_map().is_some()
    }

    fn access(
        &self,
        accessor: &mut Accessor,
    ) -> Result<Option<&dyn ConfigElement>, ConfigObjectAccessError> {
        match accessor.current() {
            Some(AccessType::Key(k)) if self.is_null() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnNull(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_bool() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnBool(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_i8() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI8(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_i16() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI16(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_i32() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI32(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_i64() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnI64(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_u8() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU8(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_u16() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU16(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_u32() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU32(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_u64() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnU64(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_f32() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnF32(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_f64() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnF64(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_str() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnStr(k.to_string()))
            }
            Some(AccessType::Key(k)) if self.is_list() => {
                Err(ConfigObjectAccessError::AccessWithKeyOnList(k.to_string()))
            }
            Some(AccessType::Key(k)) => {
                if let Some(hm) = self.as_map() {
                    if let Some(value) = hm.get(k.as_str()) {
                        accessor.advance();
                        if accessor.current().is_none() {
                            Ok(Some(value))
                        } else {
                            value.access(accessor)
                        }
                    } else {
                        Ok(None)
                    }
                } else {
                    unreachable!()
                }
            }

            Some(AccessType::Index(u)) if self.is_null() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnNull(*u))
            }
            Some(AccessType::Index(u)) if self.is_bool() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnBool(*u))
            }
            Some(AccessType::Index(u)) if self.is_i8() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI8(*u))
            }
            Some(AccessType::Index(u)) if self.is_i16() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI16(*u))
            }
            Some(AccessType::Index(u)) if self.is_i32() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI32(*u))
            }
            Some(AccessType::Index(u)) if self.is_i64() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnI64(*u))
            }
            Some(AccessType::Index(u)) if self.is_u8() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU8(*u))
            }
            Some(AccessType::Index(u)) if self.is_u16() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU16(*u))
            }
            Some(AccessType::Index(u)) if self.is_u32() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU32(*u))
            }
            Some(AccessType::Index(u)) if self.is_u64() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnU64(*u))
            }
            Some(AccessType::Index(u)) if self.is_f32() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnF32(*u))
            }
            Some(AccessType::Index(u)) if self.is_f64() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnF64(*u))
            }
            Some(AccessType::Index(u)) if self.is_str() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnStr(*u))
            }
            Some(AccessType::Index(u)) if self.is_map() => {
                Err(ConfigObjectAccessError::AccessWithIndexOnMap(*u))
            }
            Some(AccessType::Index(u)) => {
                if let Some(list) = self.as_list() {
                    if let Some(value) = list.at_index(*u) {
                        accessor.advance();
                        if accessor.current().is_none() {
                            Ok(Some(value))
                        } else {
                            value.access(accessor)
                        }
                    } else {
                        Ok(None)
                    }
                } else {
                    unreachable!()
                }
            }

            None => Err(ConfigObjectAccessError::NoAccessor),
        }
    }
}

downcast_rs::impl_downcast!(sync ConfigElement);

static_assertions::assert_obj_safe!(ConfigElement);

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "toml")]
pub mod toml;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "toml")]
    fn test_nested_toml_config() {
        use crate::Config;

        let toml: toml::Value = toml::from_str(
            r#"
            key1 = "value2"

            [table]
            key2 = "value3"
        "#,
        )
        .unwrap();

        let source = crate::source::test_source::TestSource(toml);

        let c = Config::builder().load(Box::new(source)).build().unwrap();

        let r = c.get("key1");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "value2");

        let r = c.get("table.key2");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "value3");
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_nested_toml_config_with_index() {
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

        let source = crate::source::test_source::TestSource(toml);

        let c = Config::builder().load(Box::new(source)).build().unwrap();

        let r = c.get("key.0.k");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.is_str());
        assert_eq!(r.as_str().unwrap(), "a");
    }
}
