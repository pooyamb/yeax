use std::collections::BTreeSet;

use serde::Deserialize;

use super::deserialize_filter_set;

#[derive(Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum StringFilterKind {
    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),
}

#[derive(Debug, Default, Deserialize)]
#[serde(transparent)]
pub struct StringFilter(
    #[serde(deserialize_with = "deserialize_filter_set")] pub(crate) BTreeSet<StringFilterKind>,
);
