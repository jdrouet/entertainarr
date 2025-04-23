#[macro_export]
macro_rules! json_entity_type {
    ($name:ident, $value: expr) => {
        pub const KIND: &str = $value;

        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(KIND)
            }
        }

        impl<'de> serde::de::Visitor<'de> for $name {
            type Value = $name;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "expected {KIND:?}")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == KIND {
                    Ok($name)
                } else {
                    Err(E::invalid_value(serde::de::Unexpected::Str(v), &KIND))
                }
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_str($name)
            }
        }
    };
}
