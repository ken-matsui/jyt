pub mod error;

mod c_api;
mod ext;

pub use c_api::*;
pub use ext::Ext;

pub trait Converter {
    fn to_json(self, from: Ext) -> Result<String, error::Error>;
    fn to_yaml(self, from: Ext) -> Result<String, error::Error>;
    fn to_toml(self, from: Ext) -> Result<String, error::Error>;
}

impl Converter for String {
    fn to_json(self, from: Ext) -> Result<String, error::Error> {
        let value = ext::deserialize::<ext::json::Value>(from, &self)?;
        ext::json::serialize(&value)
    }
    fn to_yaml(self, from: Ext) -> Result<String, error::Error> {
        let value = ext::deserialize::<ext::yaml::Value>(from, &self)?;
        ext::yaml::serialize(&value)
    }
    fn to_toml(self, from: Ext) -> Result<String, error::Error> {
        let value = ext::deserialize::<ext::toml::Value>(from, &self)?;
        ext::toml::serialize(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use c_api::tests::{JSON, TOML, YAML};
    use paste::paste;

    macro_rules! test_convert {
        ($from:ident, $to:ident) => {
            paste! {
                #[test]
                fn [< convert_ $from _to_ $to >]() {
                    let output = [<$from:upper>].to_string().[< to_ $to >](Ext::[<$from:camel>]);
                    assert!(output.is_ok());
                    assert_eq!(output.unwrap(), [<$to:upper>].to_string());
                }
            }
        };
    }

    test_convert!(json, yaml);
    test_convert!(json, toml);
    test_convert!(yaml, json);
    test_convert!(yaml, toml);
    test_convert!(toml, json);
    test_convert!(toml, yaml);
}
