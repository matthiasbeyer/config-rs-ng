use crate::element::ConfigElement;

use serde_json::Value;

use super::{ConfigElementListType, ConfigElementMapType};

impl ConfigElement for Value {
    fn is_null(&self) -> bool {
        std::matches!(self, Value::Null)
    }

    fn is_bool(&self) -> bool {
        std::matches!(self, Value::Bool(_))
    }

    fn is_i8(&self) -> bool {
        self.as_i64().filter(|i| *i < (i8::MAX as i64)).is_some()
    }

    fn is_i16(&self) -> bool {
        self.as_i64().filter(|i| *i < (i16::MAX as i64)).is_some()
    }

    fn is_i32(&self) -> bool {
        self.as_i64().filter(|i| *i < (i32::MAX as i64)).is_some()
    }

    fn is_i64(&self) -> bool {
        self.as_i64().filter(|i| *i < (i64::MAX as i64)).is_some()
    }

    fn is_u8(&self) -> bool {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| *i < (u8::MAX as i64))
            .is_some()
    }

    fn is_u16(&self) -> bool {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| *i < (u16::MAX as i64))
            .is_some()
    }

    fn is_u32(&self) -> bool {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| *i < (u32::MAX as i64))
            .is_some()
    }

    fn is_u64(&self) -> bool {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| *i < (u64::MAX as i64))
            .is_some()
    }

    fn is_f32(&self) -> bool {
        self.is_f64()
            && self
                .as_f64()
                .filter(|f| *f < (f32::MAX as f64) && *f > (f32::MIN as f64))
                .is_some()
    }

    fn is_f64(&self) -> bool {
        self.is_f64()
    }

    fn is_str(&self) -> bool {
        std::matches!(self, Value::String(_))
    }

    fn is_list(&self) -> bool {
        std::matches!(self, Value::Array(_))
    }

    fn is_map(&self) -> bool {
        std::matches!(self, Value::Object(_))
    }

    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }

    fn as_i8(&self) -> Option<i8> {
        self.as_i64()
            .filter(|i| (*i < (i8::MAX as i64)))
            .map(|i| i as i8)
    }

    fn as_i16(&self) -> Option<i16> {
        self.as_i64()
            .filter(|i| (*i < (i16::MAX as i64)))
            .map(|i| i as i16)
    }

    fn as_i32(&self) -> Option<i32> {
        self.as_i64()
            .filter(|i| (*i < (i32::MAX as i64)))
            .map(|i| i as i32)
    }

    fn as_i64(&self) -> Option<i64> {
        self.as_i64()
    }

    fn as_u8(&self) -> Option<u8> {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| (*i < (u8::MAX as i64)))
            .map(|i| i as u8)
    }

    fn as_u16(&self) -> Option<u16> {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| (*i < (u16::MAX as i64)))
            .map(|i| i as u16)
    }

    fn as_u32(&self) -> Option<u32> {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| (*i < (u32::MAX as i64)))
            .map(|i| i as u32)
    }

    fn as_u64(&self) -> Option<u64> {
        self.as_i64()
            .filter(|i| i.is_positive())
            .filter(|i| (*i < (u64::MAX as i64)))
            .map(|i| i as u64)
    }

    fn as_f32(&self) -> Option<f32> {
        self.as_f64()
            .filter(|f| *f < (f32::MAX as f64) && *f > (f32::MIN as f64))
            .map(|f| f as f32)
    }

    fn as_f64(&self) -> Option<f64> {
        self.as_f64()
    }

    fn as_str(&self) -> Option<&str> {
        self.as_str()
    }

    fn as_list(&self) -> Option<&dyn ConfigElementListType> {
        self.as_array().map(|a| a as &dyn ConfigElementListType)
    }

    fn as_map(&self) -> Option<&dyn ConfigElementMapType> {
        self.as_object().map(|t| t as &dyn ConfigElementMapType)
    }
}

impl ConfigElementMapType for serde_json::Map<String, serde_json::Value> {
    fn get(&self, key: &str) -> Option<&dyn ConfigElement> {
        serde_json::Map::get(self, key).map(|t| t as &dyn ConfigElement)
    }

    fn keys(&self) -> Vec<String> {
        serde_json::Map::keys(self).map(String::to_owned).collect()
    }

    fn values(&self) -> Vec<&dyn ConfigElement> {
        serde_json::Map::values(self)
            .map(|t| t as &dyn ConfigElement)
            .collect()
    }
}
