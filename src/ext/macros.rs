macro_rules! impl_serde {
    ($id:ident, $f:ident) => {
        use crate::error;

        use serde::de;
        use serde::ser;

        pub use $id::Value;

        pub(crate) fn serialize<V: ser::Serialize>(v: V) -> Result<String, error::Error> {
            Ok($id::$f(&v).map_err(|e| error::Error::Serialization(e.to_string()))?)
        }

        pub(crate) fn deserialize<V>(s: &str) -> Result<V, error::Error>
        where
            V: for<'de> de::Deserialize<'de>,
        {
            Ok($id::from_str(s).map_err(|e| error::Error::Deserialization(e.to_string()))?)
        }
    };
}

pub(crate) use impl_serde;
