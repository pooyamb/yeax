use std::collections::BTreeSet;

use chrono::NaiveDate;
use serde::Deserialize;

use super::deserialize_filter_set;

#[derive(Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum DateFilterKind {
    Before(NaiveDate),
    #[serde(rename = "eq")]
    Equals(NaiveDate),
    #[serde(rename = "neq")]
    NotEquals(NaiveDate),
    After(NaiveDate),
}

#[derive(Debug, Deserialize, Default)]
#[serde(transparent)]
pub struct DateFilter(
    #[serde(deserialize_with = "deserialize_filter_set")] pub(crate) BTreeSet<DateFilterKind>,
);
