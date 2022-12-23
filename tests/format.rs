#![cfg(not(feature = "async"))]
use config_rs_ng::Config;
use config_rs_ng::ConfigElement;
use config_rs_ng::ConfigElementListType;
use config_rs_ng::ConfigElementMapType;
use config_rs_ng::JsonFormatParser;
use config_rs_ng::StringSource;

// JSON configuration
const CONFIGURATION_LAYER_A: &str = r#"
{
    "key1": "valueA",
    "key2": "valueA"
}
"#;

// custom configuration format
const CONFIGURATION_LAYER_B: &str = r#"
mycustomformatkey(2) = valueB
mycustomformatkey(3) = valueB
"#;

// A value in our custom config format
#[derive(Debug)]
pub enum CustomValue {
    Str(String),
    Map(Map),
    // possibly more
}

// A map of values in our custom config format
#[derive(Debug)]
pub struct Map(std::collections::HashMap<String, CustomValue>);

impl ConfigElementMapType for Map {
    fn get(&self, key: &str) -> Option<&dyn ConfigElement> {
        self.0.get(key).map(|v| v as &dyn ConfigElement)
    }

    fn keys(&self) -> Vec<String> {
        self.0.keys().map(String::clone).collect()
    }

    fn values(&self) -> Vec<&dyn ConfigElement> {
        self.0.values().map(|v| v as &dyn ConfigElement).collect()
    }
}

// We need to implement ConfigElement for our custom config value type
impl config_rs_ng::ConfigElement for CustomValue {
    fn as_bool(&self) -> Option<bool> {
        None
    }

    fn as_i8(&self) -> Option<i8> {
        None
    }

    fn as_i16(&self) -> Option<i16> {
        None
    }

    fn as_i32(&self) -> Option<i32> {
        None
    }

    fn as_i64(&self) -> Option<i64> {
        None
    }

    fn as_u8(&self) -> Option<u8> {
        None
    }

    fn as_u16(&self) -> Option<u16> {
        None
    }

    fn as_u32(&self) -> Option<u32> {
        None
    }

    fn as_u64(&self) -> Option<u64> {
        None
    }

    fn as_f32(&self) -> Option<f32> {
        None
    }

    fn as_f64(&self) -> Option<f64> {
        None
    }

    fn as_str(&self) -> Option<&str> {
        if let CustomValue::Str(ref s) = self {
            Some(s)
        } else {
            None
        }
    }

    fn as_list(&self) -> Option<&dyn ConfigElementListType> {
        None
    }

    fn as_map(&self) -> Option<&dyn ConfigElementMapType> {
        if let CustomValue::Map(map) = self {
            Some(map as &dyn ConfigElementMapType)
        } else {
            None
        }
    }

    fn is_str(&self) -> bool {
        std::matches!(self, CustomValue::Str(_))
    }

    fn is_null(&self) -> bool {
        false
    }
}

// Helper type, not relevant for this test
#[derive(Debug, thiserror::Error)]
pub enum CustomError {}

impl From<CustomError> for config_rs_ng::SourceError {
    fn from(_: CustomError) -> config_rs_ng::SourceError {
        unimplemented!()
    }
}

// A parser for our custom config format
#[derive(Debug)]
struct CustomFormatParser;

impl config_rs_ng::FormatParser for CustomFormatParser {
    type Output = CustomValue;

    // This parser implementation is just a quick-and-dirty one, that works only for the exact
    // example config we wrote down above
    fn parse(buffer: Vec<u8>) -> Result<Self::Output, config_rs_ng::SourceError> {
        let s = String::from_utf8(buffer.to_vec()).unwrap();
        let hm = s
            .lines()
            .skip_while(|l| l.is_empty())
            .map(|line| {
                let num = line
                    .split('(')
                    .nth(1)
                    .expect("Splitting at '('")
                    .split(')')
                    .next()
                    .expect("Element after '('");
                let key = format!("key{}", num);
                let val = line.split(" = ").nth(1).unwrap().to_string();
                (key, CustomValue::Str(val))
            })
            .collect();
        Ok(CustomValue::Map(Map(hm)))
    }
}

#[test]
fn test_format_custom() {
    // Lets build a configuration object
    let config = Config::builder()
        // Lets load CONFIGURATION_LAYER_A from string using a JSON format parser that was shipped
        // with config-rs
        .load(Box::new({
            StringSource::<JsonFormatParser>::new(CONFIGURATION_LAYER_A.to_string())
                .expect("building StringSource")
        }))
        // Lets load CONFIGURATION_LAYER_B from string using our custom format parser that we have
        // implemented above
        .load(Box::new({
            StringSource::<CustomFormatParser>::new(CONFIGURATION_LAYER_B.to_string())
                .expect("building StringSource")
        }))
        .build()
        .expect("Building configuration object");

    // Now lets access our configuration object

    {
        // Lets get a serde_json::Value object by accessing "key1" from above
        if let Some(json_object) = config
            .layers()
            .get_as::<_, serde_json::Value>("key1")
            .expect("Accessing configuration object")
        {
            let json_object: &serde_json::Value = json_object; // enforce type
            assert!(json_object.is_string());
            assert_eq!(json_object.as_str().unwrap(), "valueA");
        } else {
            panic!("Expected 'key1' to hold a JSON object");
        }
    }
    {
        // Lets get a CustomValue object by accessing "key2" from above
        //
        // This value has shadowed the "key2" from the JSON part of our config object!
        if let Some(custom_object) = config
            .layers()
            .get_as::<_, CustomValue>("key2")
            .expect("Accessing configuration object")
        {
            let custom_object: &CustomValue = custom_object; // enforce type
            assert!(custom_object.is_str());
            assert_eq!(custom_object.as_str().unwrap(), "valueB");
        } else {
            panic!("Expected 'key2' to hold a CUSTOM object");
        }
    }
}
