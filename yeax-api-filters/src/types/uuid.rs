use std::collections::BTreeSet;

use serde::Deserialize;
use uuid::Uuid;

use super::deserialize_filter_set;

#[derive(Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub enum UuidFilterKind {
    #[serde(rename = "eq")]
    Equals(Uuid),
    #[serde(rename = "in")]
    In(Vec<Uuid>),
}

#[derive(Debug, Deserialize, Default)]
#[serde(transparent)]
pub struct UuidFilter(
    #[serde(deserialize_with = "deserialize_filter_set")] pub(crate) BTreeSet<UuidFilterKind>,
);
