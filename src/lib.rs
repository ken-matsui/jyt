pub mod error;

mod ext;

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
    use paste::paste;

    static JSON: &str = r#"{
  "title": "TOML Example",
  "owner": {
    "name": "Tom Preston-Werner"
  },
  "database": {
    "server": "192.168.1.1",
    "ports": [
      8000,
      8001,
      8002
    ],
    "connection_max": 5000,
    "enabled": true
  }
}"#;

    static YAML: &str = r#"title: TOML Example
owner:
  name: Tom Preston-Werner
database:
  server: 192.168.1.1
  ports:
  - 8000
  - 8001
  - 8002
  connection_max: 5000
  enabled: true
"#;

    static TOML: &str = r#"title = "TOML Example"

[owner]
name = "Tom Preston-Werner"

[database]
server = "192.168.1.1"
ports = [8000, 8001, 8002]
connection_max = 5000
enabled = true
"#;

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
