use core::fmt;
use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub enum FilterExpr<T> {
    Eq(T),
    Neq(T),
    Gt(T),
    Lt(T),
    Gte(T),
    Lte(T),
}

impl<'a, T> FilterExpr<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Display + Sync + Send + 'static,
{
    fn parse(value: &'a str) -> anyhow::Result<Self> {
        if let Some(rest) = value.strip_prefix(">=") {
            let value = rest.parse().map_err(|e| anyhow!("Parse error: `{e}`"))?;
            Ok(FilterExpr::Gte(value))
        } else if let Some(rest) = value.strip_prefix("<=") {
            let value = rest.parse().map_err(|e| anyhow!("Parse error: `{e}`"))?;
            Ok(FilterExpr::Lte(value))
        } else if let Some(rest) = value.strip_prefix(">") {
            let value = rest.parse().map_err(|e| anyhow!("Parse error: `{e}`"))?;
            Ok(FilterExpr::Gt(value))
        } else if let Some(rest) = value.strip_prefix("<") {
            let value = rest.parse().map_err(|e| anyhow!("Parse error: `{e}`"))?;
            Ok(FilterExpr::Lt(value))
        } else if let Some(rest) = value.strip_prefix("^=") {
            let value = rest.parse().map_err(|e| anyhow!("Parse error: `{e}`"))?;
            Ok(FilterExpr::Neq(value))
        } else if let Some(rest) = value.strip_prefix("=") {
            let value = rest.parse().map_err(|e| anyhow!("Parse error: `{e}`"))?;
            Ok(FilterExpr::Eq(value))
        } else {
            let value = value.parse().map_err(|e| anyhow!("Parse error: `{e}`"))?;
            Ok(FilterExpr::Lt(value))
        }
    }
}

impl<T> FilterExpr<T>
where
    T: PartialEq + PartialOrd,
{
    pub fn apply(&self, other: T) -> bool {
        match self {
            FilterExpr::Eq(x) => other == *x,
            FilterExpr::Neq(x) => other != *x,
            FilterExpr::Gt(x) => other > *x,
            FilterExpr::Lt(x) => other < *x,
            FilterExpr::Gte(x) => other >= *x,
            FilterExpr::Lte(x) => other <= *x,
        }
    }

    pub fn apply_as<S>(&self, other: S) -> bool
    where
        S: Into<T>,
    {
        match self {
            FilterExpr::Eq(x) => other.into() == *x,
            FilterExpr::Neq(x) => other.into() != *x,
            FilterExpr::Gt(x) => other.into() > *x,
            FilterExpr::Lt(x) => other.into() < *x,
            FilterExpr::Gte(x) => other.into() >= *x,
            FilterExpr::Lte(x) => other.into() <= *x,
        }
    }
}

impl<T> FromStr for FilterExpr<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + fmt::Display + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FilterExpr::<T>::parse(s)
    }
}

#[cfg(feature = "serde")]
use serde::de;

#[cfg(feature = "serde")]
use std::marker::PhantomData;

#[cfg(feature = "serde")]
impl<'de, T> de::Deserialize<'de> for FilterExpr<T>
where
    T: FromStr + PartialEq + PartialOrd,
    <T as FromStr>::Err: Sync + Send + Into<anyhow::Error> + std::error::Error + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<S>(PhantomData<S>);

        impl<'de, S> de::Visitor<'de> for Visitor<S>
        where
            S: FromStr + PartialEq + PartialOrd,
            <S as FromStr>::Err: Sync + Send + Into<anyhow::Error> + std::error::Error + 'static,
        {
            type Value = FilterExpr<S>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("valid url")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let phone_number = FilterExpr::parse(value).map_err(|_| {
                    E::custom(format!(
                        "Supplied value ({value}) cannot be parsed as EQ filter"
                    ))
                })?;
                Ok(phone_number)
            }
        }

        deserializer.deserialize_string(Visitor::<T>(Default::default()))
    }
}

#[cfg(all(test, feature = "serde"))]
mod test {
    use serde::Deserialize;

    use assert_matches::assert_matches;

    use crate::filter_expr::FilterExpr;

    #[test]
    fn test_gt() {
        #[derive(Deserialize)]
        struct Filter {
            value: FilterExpr<f32>,
        }
        let s = "{\"value\": \">2\"}";
        let item: Filter = serde_json::from_str(s).unwrap();

        let v = assert_matches!(item.value, FilterExpr::Gt(x) => x);

        assert_eq!(v, 2_f32);
    }
    #[test]
    fn test_gte() {
        #[derive(Deserialize)]
        struct Filter {
            value: FilterExpr<i32>,
        }
        let s = "{\"value\": \">=-2\"}";
        let item: Filter = serde_json::from_str(s).unwrap();

        let v = assert_matches!(item.value, FilterExpr::Gte(x) => x);

        assert_eq!(v, -2);
    }
}
