use entertainarr_domain::prelude::SortOrder;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct Page {
    #[serde(default = "Page::default_limit")]
    pub limit: u32,
    #[serde(default = "Page::default_offset")]
    pub offset: u32,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            limit: Self::default_limit(),
            offset: Self::default_offset(),
        }
    }
}

impl Page {
    pub const fn default_limit() -> u32 {
        50
    }

    pub const fn default_offset() -> u32 {
        0
    }
}

impl From<Page> for entertainarr_domain::prelude::Page {
    fn from(value: Page) -> Self {
        Self {
            limit: value.limit,
            offset: value.offset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sort<T> {
    field: T,
    order: SortOrder,
}

impl<A, B> From<Sort<A>> for entertainarr_domain::prelude::Sort<B>
where
    A: Into<B>,
{
    fn from(value: Sort<A>) -> Self {
        Self {
            field: value.field.into(),
            order: value.order,
        }
    }
}

impl<T: Default> Default for Sort<T> {
    fn default() -> Self {
        Self {
            field: T::default(),
            order: SortOrder::Asc,
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for Sort<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if let Some(value) = value.strip_prefix("-") {
            Ok(Self {
                field: T::from_str(value).map_err(serde::de::Error::custom)?,
                order: SortOrder::Desc,
            })
        } else {
            let value = value.trim_start_matches("+");
            Ok(Self {
                field: T::from_str(value).map_err(serde::de::Error::custom)?,
                order: SortOrder::Asc,
            })
        }
    }
}

pub fn from_comma_separated<'de, D, C, T>(deserializer: D) -> Result<C, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    C: FromIterator<T>,
{
    let input = String::deserialize(deserializer)?;
    input
        .split(',')
        .map(|cell| cell.parse::<T>().map_err(serde::de::Error::custom))
        .collect()
}
