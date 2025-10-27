use crux_core::Command;

#[derive(Clone, Debug, facet::Facet, serde::Serialize, serde::Deserialize)]
#[repr(C)]
pub enum Persistence {
    Clear(ClearEffect),
    Store(StoreEffect),
}

impl Persistence {
    pub fn store(key: impl Into<String>, value: impl Into<String>) -> crate::ApplicationCommand {
        Command::notify_shell(Self::Store(StoreEffect {
            key: key.into(),
            value: value.into(),
        }))
        .into()
    }
    pub fn clear(key: impl Into<String>) -> crate::ApplicationCommand {
        Command::notify_shell(Self::Clear(ClearEffect { key: key.into() })).into()
    }
}

impl crux_core::capability::Operation for Persistence {
    type Output = ();

    #[cfg(feature = "typegen")]
    fn register_types(
        generator: &mut crux_core::type_generation::serde::TypeGen,
    ) -> crux_core::type_generation::serde::Result
    where
        Self: serde::Serialize + for<'de> serde::de::Deserialize<'de>,
        Self::Output: for<'de> serde::de::Deserialize<'de>,
    {
        generator.register_type::<StoreEffect>()?;
        generator.register_type::<Self>()?;
        Ok(())
    }
}

#[derive(facet::Facet, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ClearEffect {
    pub key: String,
}

#[derive(facet::Facet, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct StoreEffect {
    pub key: String,
    pub value: String,
}
