use std::fmt;

use serde::Deserialize;

pub mod tosqlx;
pub mod types;

#[derive(Deserialize, Debug)]
pub struct QueryFilter<T = ()> {
    start: Option<u32>,
    end: Option<u32>,
    sort: Option<String>,
    order: Option<String>,

    #[serde(default)]
    filter: T,
}

impl<T> QueryFilter<T>
where
    T: Filter,
{
    pub fn get_offset(&self) -> u32 {
        // Check if start is more than 0
        if let Some(offset) = self.start {
            offset
        } else {
            0
        }
    }

    pub fn get_limit(&self, offset: u32) -> u32 {
        if let Some(value) = self.end {
            std::cmp::min(std::cmp::max(value - offset, 1), T::get_max_limit())
        } else {
            10
        }
    }

    pub fn get_sort(&self) -> Option<&str> {
        if let Some(ref field) = self.sort {
            if T::validate_sortable_field(field) {
                Some(field.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_order(&self) -> Order {
        if let Some(ref val) = self.order {
            val.parse().unwrap_or(Order::None)
        } else {
            Order::None
        }
    }

    pub fn get_filter(&self) -> &T {
        &self.filter
    }
}

pub trait Filter {
    const SORTABLE_FIELDS: &'static [&'static str];

    fn validate_sortable_field(field: &str) -> bool {
        Self::SORTABLE_FIELDS.contains(&field)
    }

    fn get_max_limit() -> u32 {
        100
    }
}

pub enum Order {
    Asc,
    Desc,
    None,
}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
            _ => "ASC",
        }
    }
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Order {
    type Err = ();
    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val.to_ascii_uppercase().as_str() {
            "ASC" => Ok(Order::Asc),
            "DESC" => Ok(Order::Desc),
            _ => Err(()),
        }
    }
}
