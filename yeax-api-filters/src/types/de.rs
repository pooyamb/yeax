use std::collections::BTreeSet;
use std::fmt;
use std::marker::PhantomData;

use serde::{de, de::value::MapAccessDeserializer};

/// A little hacky deserializer that flattens a map of key-value pairs to variant-value for enums
/// It is only working with serde-querystring, due to the way it proccesses maps, being more forgiving that serde-json
pub(crate) fn deserialize_filter_set<'de, D, T>(deserializer: D) -> Result<BTreeSet<T>, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::Deserialize<'de> + Eq + Ord,
{
    deserializer.deserialize_map(SomeVisitor(PhantomData))
}

struct SomeVisitor<T>(PhantomData<T>);

impl<'de, T> de::Visitor<'de> for SomeVisitor<T>
where
    T: de::Deserialize<'de>,
    T: Eq + Ord,
{
    type Value = BTreeSet<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Sequence of filters")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut temp = BTreeSet::new();

        for _ in 0..map.size_hint().unwrap_or(0) {
            temp.insert(T::deserialize(MapAccessDeserializer::new(&mut map))?);
        }

        Ok(temp)
    }
}
