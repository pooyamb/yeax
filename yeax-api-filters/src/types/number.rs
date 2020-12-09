use std::collections::BTreeSet;

use serde::Deserialize;

use super::deserialize_filter_set;

#[derive(Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum NumberFilterKind {
    #[serde(rename = "eq")]
    Equals(i64),
    #[serde(rename = "neq")]
    NotEquals(i64),
    #[serde(rename = "lt")]
    LesserThan(i64),
    #[serde(rename = "lte")]
    LesserThanEqual(i64),
    #[serde(rename = "gt")]
    GreaterThan(i64),
    #[serde(rename = "gte")]
    GreaterThanEqual(i64),
}

#[derive(Debug, Deserialize, Default)]
#[serde(transparent)]
pub struct NumberFilter(
    #[serde(deserialize_with = "deserialize_filter_set")] pub(crate) BTreeSet<NumberFilterKind>,
);
