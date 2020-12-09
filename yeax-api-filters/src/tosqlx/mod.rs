//! A set of traits and impls for converting filters into sqlx queries
//! Although it's generic over the provided database type wise, it only works with postgres
//! due to prepared query placeholder syntax used in it.
//! It may also make sense to use some orm or query builder instead of generating queries directly.

use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{Arguments, Encode, Type};

use crate::Filter;

use super::types::*;
use super::QueryFilter;

use uuid::Uuid;

pub fn to_sql<'a, 'b, T, A>(
    filters: &'b T,
    query: &'b mut String,
    args: &'b mut A,
    start_index: u32,
) -> u32
where
    'b: 'a,
    &'b T: ToSql<'a, A>,
    A: Arguments<'a>,
{
    debug_assert!(start_index != 0);
    *query += " 1=1 AND ";
    let mut config = Config::new(query, args, start_index);
    config.check(filters);
    config.get_index()
}

pub struct Config<'a, A> {
    pub field_name: &'static str,
    pub query: &'a mut String,
    pub args: &'a mut A,
    pub index: u32,
    pub used: bool,
}

impl<'a, A> Config<'a, A> {
    pub fn new(query: &'a mut String, args: &'a mut A, start_index: u32) -> Self {
        debug_assert!(start_index != 0);
        Self {
            query,
            args,
            index: start_index,
            field_name: "",
            used: false,
        }
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }
}

impl<'a, 'b, A> Config<'a, A>
where
    A: Arguments<'b>,
{
    pub fn check<V>(&mut self, value: V)
    where
        V: ToSql<'a, A> + 'a,
    {
        value.to_sql(self);
    }

    pub fn check_field<V>(&mut self, field_name: &'static str, value: V)
    where
        V: ToSql<'b, A>,
    {
        if self.used {
            *self.query += " AND ";
        }
        let mut conf = Config::new(self.query, self.args, self.index);
        conf.field_name = field_name;
        value.to_sql(&mut conf);
        self.index = conf.index;
        self.used = conf.used;
    }
}

pub trait ToSql<'b, A> {
    fn to_sql(self, config: &mut Config<A>);
}

impl<'b, A> ToSql<'b, A> for &'b NumberFilterKind
where
    A: Arguments<'b>,
    i64: Encode<'b, A::Database>,
    i64: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        *config.query += config.field_name;
        match self {
            NumberFilterKind::Equals(val) => {
                *config.query += "==$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            NumberFilterKind::NotEquals(val) => {
                *config.query += "=!$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            NumberFilterKind::GreaterThan(val) => {
                *config.query += ">$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            NumberFilterKind::GreaterThanEqual(val) => {
                *config.query += ">=$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            NumberFilterKind::LesserThan(val) => {
                *config.query += "<$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            NumberFilterKind::LesserThanEqual(val) => {
                *config.query += "<=$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
        };
        config.index += 1;
        config.used = true;
    }
}

impl<'b, A> ToSql<'b, A> for &'b NumberFilter
where
    A: Arguments<'b>,
    i64: Encode<'b, A::Database>,
    i64: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        for (index, filter) in self.0.iter().enumerate() {
            filter.to_sql(config);
            if index + 1 != self.0.len() {
                *config.query += " AND ";
            }
        }
    }
}

impl<'b, A> ToSql<'b, A> for &'b DateFilterKind
where
    A: Arguments<'b>,
    NaiveDate: Encode<'b, A::Database>,
    NaiveDate: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        *config.query += config.field_name;
        match self {
            DateFilterKind::Equals(val) => {
                *config.query += "==$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            DateFilterKind::NotEquals(val) => {
                *config.query += "=!$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            DateFilterKind::Before(val) => {
                *config.query += "<$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            DateFilterKind::After(val) => {
                *config.query += ">$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
        };
        config.index += 1;
        config.used = true;
    }
}

impl<'b, A> ToSql<'b, A> for &'b DateFilter
where
    A: Arguments<'b>,
    NaiveDate: Encode<'b, A::Database>,
    NaiveDate: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        for (index, filter) in self.0.iter().enumerate() {
            filter.to_sql(config);
            if index + 1 != self.0.len() {
                *config.query += " AND ";
            }
        }
    }
}

impl<'b, A> ToSql<'b, A> for &'b DateTimeFilterKind
where
    A: Arguments<'b>,
    NaiveDateTime: Encode<'b, A::Database>,
    NaiveDateTime: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        *config.query += config.field_name;
        match self {
            DateTimeFilterKind::Equals(val) => {
                *config.query += "==$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            DateTimeFilterKind::NotEquals(val) => {
                *config.query += "=!$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            DateTimeFilterKind::Before(val) => {
                *config.query += "<$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
            DateTimeFilterKind::After(val) => {
                *config.query += ">$";
                *config.query += &config.index.to_string();
                config.args.add(val);
            }
        };
        config.index += 1;
        config.used = true;
    }
}

impl<'b, A> ToSql<'b, A> for &'b DateTimeFilter
where
    A: Arguments<'b>,
    NaiveDateTime: Encode<'b, A::Database>,
    NaiveDateTime: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        for (index, filter) in self.0.iter().enumerate() {
            filter.to_sql(config);
            if index + 1 != self.0.len() {
                *config.query += " AND ";
            }
        }
    }
}

impl<'b, A> ToSql<'b, A> for &'b StringFilterKind
where
    A: Arguments<'b>,
    String: Encode<'b, A::Database>,
    String: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        *config.query += config.field_name;
        match self {
            StringFilterKind::Contains(val) => {
                *config.query += " LIKE $";
                *config.query += &config.index.to_string();
                println!("{:?}", ["%", val, "%"].join(""));
                config.args.add(["%", val, "%"].join(""));
            }
            StringFilterKind::NotContains(val) => {
                *config.query += " NOT LIKE $";
                *config.query += &config.index.to_string();
                println!("{:?}", ["%", val, "%"].join(""));
                config.args.add(["%", val, "%"].join(""));
            }
            StringFilterKind::StartsWith(val) => {
                *config.query += " LIKE $";
                *config.query += &config.index.to_string();
                println!("{:?}", [val, "%"].join(""));
                config.args.add([val, "%"].join(""));
            }
            StringFilterKind::EndsWith(val) => {
                *config.query += " LIKE $";
                *config.query += &config.index.to_string();
                println!("{:?}", ["%", val].join(""));
                config.args.add(["%", val].join(""));
            }
        };
        config.index += 1;
        config.used = true;
    }
}

impl<'b, A> ToSql<'b, A> for &'b StringFilter
where
    A: Arguments<'b>,
    String: Encode<'b, A::Database>,
    String: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        for (index, filter) in self.0.iter().enumerate() {
            filter.to_sql(config);
            if index + 1 != self.0.len() {
                *config.query += " AND ";
            }
        }
    }
}

impl<'b, A> ToSql<'b, A> for &'b UuidFilterKind
where
    A: Arguments<'b>,
    Uuid: Encode<'b, A::Database>,
    Uuid: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        *config.query += config.field_name;
        match self {
            UuidFilterKind::Equals(val) => {
                *config.query += "= $";
                *config.query += &config.index.to_string();
                config.args.add(val);
                config.index += 1;
            }
            UuidFilterKind::In(val) => {
                *config.query += " IN (";
                for (index, uuid) in val.iter().enumerate() {
                    *config.query += "$";
                    *config.query += &config.index.to_string();
                    config.args.add(uuid);
                    config.index += 1;

                    if index + 1 < val.len() {
                        *config.query += ",";
                    }
                }
                *config.query += ")";
            }
        };
        config.used = true;
    }
}

impl<'b, A> ToSql<'b, A> for &'b UuidFilter
where
    A: Arguments<'b>,
    Uuid: Encode<'b, A::Database>,
    Uuid: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        for (index, filter) in self.0.iter().enumerate() {
            filter.to_sql(config);
            if index + 1 != self.0.len() {
                *config.query += " AND ";
            }
        }
    }
}

impl<'b, T, A> ToSql<'b, A> for &'b QueryFilter<T>
where
    T: Filter,
    &'b T: ToSql<'b, A>,
    A: Arguments<'b>,
    u32: Encode<'b, A::Database>,
    u32: Type<A::Database>,
{
    fn to_sql(self, config: &mut Config<A>) {
        self.filter.to_sql(config);

        let offset = self.get_offset();
        let limit = self.get_limit(offset);
        let order = self.get_order();
        let sort = self.get_sort();

        if let Some(field) = sort {
            *config.query += " ORDER BY ";
            *config.query += field;
            *config.query += " ";
            *config.query += order.as_str();
        }

        *config.query += " OFFSET $";
        *config.query += &config.index.to_string();
        config.index += 1;

        *config.query += " LIMIT $";
        *config.query += &config.index.to_string();
        config.index += 1;

        config.used = true;

        config.args.add(offset);
        config.args.add(limit);
    }
}

impl<'b, T, A> ToSql<'b, A> for &'b Option<T>
where
    &'b T: ToSql<'b, A>,
    A: Arguments<'b>,
{
    fn to_sql(self, config: &mut Config<A>) {
        match self {
            Some(ref val) => val.to_sql(config),
            None => {}
        };
    }
}

impl<'b, A> ToSql<'b, A> for ()
where
    A: Arguments<'b>,
{
    fn to_sql<'a>(self, _: &mut Config<'a, A>) {}
}
