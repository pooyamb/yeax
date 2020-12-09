use std::collections::BTreeSet;

use chrono::NaiveDateTime;
use serde::Deserialize;

use super::deserialize_filter_set;

#[derive(Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum DateTimeFilterKind {
    Before(NaiveDateTime),
    #[serde(rename = "eq")]
    Equals(NaiveDateTime),
    #[serde(rename = "neq")]
    NotEquals(NaiveDateTime),
    After(NaiveDateTime),
}

#[derive(Debug, Deserialize, Default)]
#[serde(transparent)]
pub struct DateTimeFilter(
    #[serde(deserialize_with = "deserialize_filter_set")] pub(crate) BTreeSet<DateTimeFilterKind>,
);
