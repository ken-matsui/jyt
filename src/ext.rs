pub(crate) mod macros;

pub(crate) mod json;
pub(crate) mod toml;
pub(crate) mod yaml;

use crate::error;

#[repr(C)]
pub enum Ext {
    Json,
    Yaml,
    Toml,
}

pub(crate) fn deserialize<V>(from: Ext, s: &str) -> Result<V, error::Error>
where
    V: for<'de> serde::Deserialize<'de>,
{
    match from {
        Ext::Json => json::deserialize(s),
        Ext::Yaml => yaml::deserialize(s),
        Ext::Toml => toml::deserialize(s),
    }
}
