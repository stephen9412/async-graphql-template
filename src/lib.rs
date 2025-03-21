// Derived from Seaography (github.com/SeaQL/seaography)
// Modifications Copyright (c) 2025 Stephen J. Li

//! <div align="center">
//!
//!   <h1>
//!     <img src="https://raw.githubusercontent.com/SeaQL/seaography/main/docs/Seaography.png" width="280 alt="Seaography"/>
//!   </h1>
//!
//!   <p>
//!     <strong>🧭 A GraphQL framework and code generator for SeaORM</strong>
//!   </p>
//!
//!   [![crate](https://img.shields.io/crates/v/seaography.svg)](https://crates.io/crates/seaography)
//!   [![docs](https://docs.rs/seaography/badge.svg)](https://docs.rs/seaography)
//!   [![build status](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml/badge.svg)](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml)
//!
//! </div>
//!
//! # Seaography
//!
//! #### Seaography is a GraphQL framework for building GraphQL resolvers using SeaORM entities. It ships with a CLI tool that can generate ready-to-compile Rust GraphQL servers from existing MySQL, Postgres and SQLite databases.
//!
//! ## Features
//!
//! * Relational query (1-to-1, 1-to-N)
//! * Pagination on query's root entity
//! * Filter with operators (e.g. gt, lt, eq)
//! * Order by any column
//!
//! (Right now there is no mutation, but it's on our plan!)
//!
//! ## Quick start - ready to serve in 3 minutes!
//!
//! ### Install
//!
//! ```sh
//! cargo install seaography-cli
//! ```
//!
//! ### MySQL
//!
//! Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/mysql/sakila-schema.sql) sample database.
//!
//! ```sh
//! cd examples/mysql
//! seaography-cli mysql://user:pw@localhost/sakila seaography-mysql-example .
//! cargo run
//! ```
//!
//! Go to http://localhost:8000/ and try out the following queries:
//!
//! #### Fetch films and their actors
//!
//! ```graphql
//! {
//!   film(pagination: { limit: 10, page: 0 }, orderBy: { title: ASC }) {
//!     data {
//!       title
//!       description
//!       releaseYear
//!       filmActor {
//!         actor {
//!           firstName
//!           lastName
//!         }
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! #### Fetch store and its employee
//!
//! ```graphql
//! {
//!   store(filters: { storeId: { eq: 1 } }) {
//!     data {
//!       storeId
//!       address {
//!         address
//!         address2
//!       }
//!       staff {
//!         firstName
//!         lastName
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ### Postgres
//!
//! Setup the [sakila](https://github.com/SeaQL/seaography/blob/main/examples/postgres/sakila-schema.sql) sample database.
//!
//! ```sh
//! cd examples/postgres
//! seaography-cli postgres://user:pw@localhost/sakila seaography-postgres-example .
//! cargo run
//! ```
//!
//! ### SQLite
//!
//! ```sh
//! cd examples/sqlite
//! seaography-cli sqlite://chinook.db seaography-sqlite-example .
//! cargo run
//! ```
//!
//! Go to http://localhost:8000/ and try out the following query:
//!
//! #### Fetch albums and their artists
//!
//! ```graphql
//! {
//!   albums(pagination: { limit: 10, page: 0 }) {
//!     data {
//!       title
//!       artists {
//!         name
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
//! Seaography is a community driven project. We welcome you to participate, contribute and together build for Rust's future.

use std::{fmt::Debug, str::FromStr};

pub use async_graphql_template_derive as macros;
pub use heck;
pub use itertools;
use itertools::Itertools;

// 引入數組過濾器模塊
mod array_filter;
pub use array_filter::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
pub enum OrderByEnum {
    Asc,
    Desc,
}

pub type BinaryVector = Vec<u8>;

#[derive(Debug, Clone, async_graphql::InputObject)]
#[graphql(concrete(name = "TinyIntegerFilter", params(i8)))]
#[graphql(concrete(name = "SmallIntegerFilter", params(i16)))]
#[graphql(concrete(name = "IntegerFilter", params(i32)))]
#[graphql(concrete(name = "BigIntegerFilter", params(i64)))]
#[graphql(concrete(name = "TinyUnsignedFilter", params(u8)))]
#[graphql(concrete(name = "SmallUnsignedFilter", params(u16)))]
#[graphql(concrete(name = "UnsignedFilter", params(u32)))]
#[graphql(concrete(name = "BigUnsignedFilter", params(u64)))]
#[graphql(concrete(name = "FloatFilter", params(f32)))]
#[graphql(concrete(name = "DoubleFilter", params(f64)))]
#[cfg_attr(
    feature = "with-json",
    graphql(concrete(name = "JsonFilter", params(sea_orm::prelude::Json)))
)]
// TODO #[graphql(concrete(name = "DateFilter", params()))]
// TODO #[graphql(concrete(name = "TimeFilter", params()))]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateFilter", params(sea_orm::prelude::Date)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeFilter", params(sea_orm::prelude::DateTime)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeUtcFilter", params(sea_orm::prelude::DateTimeUtc)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(
        name = "DateTimeWithTimeZoneFilter",
        params(sea_orm::prelude::DateTimeWithTimeZone)
    ))
)]
// TODO #[graphql(concrete(name = "TimestampFilter", params()))]
// TODO #[graphql(concrete(name = "TimestampWithTimeZoneFilter", params()))]
#[cfg_attr(
    feature = "with-decimal",
    graphql(concrete(name = "DecimalFilter", params(sea_orm::prelude::Decimal)))
)]
#[cfg_attr(
    feature = "with-uuid",
    graphql(concrete(name = "UuidFilter", params(sea_orm::prelude::Uuid)))
)]
#[graphql(concrete(name = "BinaryFilter", params(BinaryVector)))]
#[graphql(concrete(name = "BooleanFilter", params(bool)))]
pub struct TypeFilter<T: async_graphql::InputType> {
    pub eq: Option<T>,
    pub ne: Option<T>,
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
    pub is_in: Option<Vec<T>>,
    pub is_not_in: Option<Vec<T>>,
    pub is_null: Option<bool>,
}

#[derive(Debug, Clone, async_graphql::InputObject)]
#[graphql(concrete(name = "StringFilter", params(String)))]
pub struct StringFilter<T: async_graphql::InputType> {
    pub like: Option<T>,
    pub eq: Option<T>,
    pub ne: Option<T>,
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
    pub is_in: Option<Vec<T>>,
    pub is_not_in: Option<Vec<T>>,
    pub is_null: Option<bool>,
}

#[derive(Debug, async_graphql::InputObject)]
pub struct PageInput {
    pub limit: usize,
    pub page: usize,
}

#[derive(Debug, async_graphql::InputObject)]
pub struct CursorInput {
    pub cursor: Option<String>,
    pub limit: u64,
}

#[derive(async_graphql::OneofObject)]
pub enum Pagination {
    Pages(PageInput),
    Cursor(CursorInput),
}

#[derive(async_graphql::SimpleObject)]
pub struct ExtraPaginationFields {
    pub pages: Option<usize>,
    pub current: Option<usize>,
}

#[derive(Debug)]
pub enum DecodeMode {
    Type,
    Length,
    ColonSkip,
    Data,
}

pub fn map_cursor_values(values: Vec<sea_orm::Value>) -> sea_orm::sea_query::value::ValueTuple {
    if values.len() == 1 {
        sea_orm::sea_query::value::ValueTuple::One(values[0].clone())
    } else if values.len() == 2 {
        sea_orm::sea_query::value::ValueTuple::Two(values[0].clone(), values[1].clone())
    } else if values.len() == 3 {
        sea_orm::sea_query::value::ValueTuple::Three(
            values[0].clone(),
            values[1].clone(),
            values[2].clone(),
        )
    } else {
        panic!("seaography does not support cursors values with size greater than 3")
    }
}

#[derive(Debug)]
pub struct CursorValues(pub Vec<sea_orm::Value>);

impl async_graphql::types::connection::CursorType for CursorValues {
    type Error = String;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        let chars = s.chars();

        let mut values: Vec<sea_orm::Value> = vec![];

        let mut type_indicator = String::new();
        let mut length_indicator = String::new();
        let mut data_buffer = String::new();
        let mut length = -1;

        let mut mode: DecodeMode = DecodeMode::Type;
        for char in chars {
            match mode {
                DecodeMode::Type => {
                    if char.eq(&'[') {
                        mode = DecodeMode::Length;
                    } else if char.eq(&',') {
                        // SKIP
                    } else {
                        type_indicator.push(char);
                    }
                }
                DecodeMode::Length => {
                    if char.eq(&']') {
                        mode = DecodeMode::ColonSkip;
                        length = length_indicator.parse::<i64>().unwrap();
                    } else {
                        length_indicator.push(char);
                    }
                }
                DecodeMode::ColonSkip => {
                    // skips ':' char
                    mode = DecodeMode::Data;
                }
                DecodeMode::Data => {
                    if length > 0 {
                        data_buffer.push(char);
                        length -= 1;
                    }

                    if length <= 0 {
                        let value: sea_orm::Value = match type_indicator.as_str() {
                            "TinyInt" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::TinyInt(None)
                                } else {
                                    sea_orm::Value::TinyInt(Some(
                                        data_buffer.parse::<i8>().unwrap(),
                                    ))
                                }
                            }
                            "SmallInt" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::SmallInt(None)
                                } else {
                                    sea_orm::Value::SmallInt(Some(
                                        data_buffer.parse::<i16>().unwrap(),
                                    ))
                                }
                            }
                            "Int" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::Int(None)
                                } else {
                                    sea_orm::Value::Int(Some(data_buffer.parse::<i32>().unwrap()))
                                }
                            }
                            "BigInt" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::BigInt(None)
                                } else {
                                    sea_orm::Value::BigInt(Some(
                                        data_buffer.parse::<i64>().unwrap(),
                                    ))
                                }
                            }
                            "TinyUnsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::TinyUnsigned(None)
                                } else {
                                    sea_orm::Value::TinyUnsigned(Some(
                                        data_buffer.parse::<u8>().unwrap(),
                                    ))
                                }
                            }
                            "SmallUnsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::SmallUnsigned(None)
                                } else {
                                    sea_orm::Value::SmallUnsigned(Some(
                                        data_buffer.parse::<u16>().unwrap(),
                                    ))
                                }
                            }
                            "Unsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::Unsigned(None)
                                } else {
                                    sea_orm::Value::Unsigned(Some(
                                        data_buffer.parse::<u32>().unwrap(),
                                    ))
                                }
                            }
                            "BigUnsigned" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::BigUnsigned(None)
                                } else {
                                    sea_orm::Value::BigUnsigned(Some(
                                        data_buffer.parse::<u64>().unwrap(),
                                    ))
                                }
                            }
                            "String" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::String(None)
                                } else {
                                    sea_orm::Value::String(Some(Box::new(
                                        data_buffer.parse::<String>().unwrap(),
                                    )))
                                }
                            }
                            #[cfg(feature = "with-uuid")]
                            "Uuid" => {
                                if length.eq(&-1) {
                                    sea_orm::Value::Uuid(None)
                                } else {
                                    sea_orm::Value::Uuid(Some(Box::new(
                                        data_buffer.parse::<sea_orm::prelude::Uuid>().unwrap(),
                                    )))
                                }
                            }
                            _ => {
                                // FIXME: missing value types
                                panic!("cannot encode current type")
                            }
                        };

                        values.push(value);

                        type_indicator = String::new();
                        length_indicator = String::new();
                        data_buffer = String::new();
                        length = -1;

                        mode = DecodeMode::Type;
                    }
                }
            }
        }

        Ok(Self(values))
    }

    fn encode_cursor(&self) -> String {
        self.0
            .iter()
            .map(|value| -> String {
                match value {
                    sea_orm::Value::TinyInt(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("TinyInt[{}]:{}", value.len(), value)
                        } else {
                            "TinyInt[-1]:".into()
                        }
                    }
                    sea_orm::Value::SmallInt(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("SmallInt[{}]:{}", value.len(), value)
                        } else {
                            "SmallInt[-1]:".into()
                        }
                    }
                    sea_orm::Value::Int(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("Int[{}]:{}", value.len(), value)
                        } else {
                            "Int[-1]:".into()
                        }
                    }
                    sea_orm::Value::BigInt(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("BigInt[{}]:{}", value.len(), value)
                        } else {
                            "BigInt[-1]:".into()
                        }
                    }
                    sea_orm::Value::TinyUnsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("TinyUnsigned[{}]:{}", value.len(), value)
                        } else {
                            "TinyUnsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::SmallUnsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("SmallUnsigned[{}]:{}", value.len(), value)
                        } else {
                            "SmallUnsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::Unsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("Unsigned[{}]:{}", value.len(), value)
                        } else {
                            "Unsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::BigUnsigned(value) => {
                        if let Some(value) = value {
                            let value = value.to_string();
                            format!("BigUnsigned[{}]:{}", value.len(), value)
                        } else {
                            "BigUnsigned[-1]:".into()
                        }
                    }
                    sea_orm::Value::String(value) => {
                        if let Some(value) = value {
                            let value = value.as_ref();
                            format!("String[{}]:{}", value.len(), value)
                        } else {
                            "String[-1]:".into()
                        }
                    }
                    #[cfg(feature = "with-uuid")]
                    sea_orm::Value::Uuid(value) => {
                        if let Some(value) = value {
                            let value = value.as_ref().to_string();
                            format!("Uuid[{}]:{}", value.len(), value)
                        } else {
                            "Uuid[-1]:".into()
                        }
                    }
                    _ => {
                        // FIXME: missing value types
                        panic!("cannot encode current type")
                    }
                }
            })
            .join(",")
    }
}

#[derive(Debug, Clone)]
pub struct RelationKeyStruct<Filter, Order>(pub sea_orm::Value, pub Filter, pub Order);

impl<Filter, Order> PartialEq for RelationKeyStruct<Filter, Order> {
    fn eq(&self, other: &Self) -> bool {
        // TODO temporary hack to solve the following problem
        // let v1 = TestFK(sea_orm::Value::TinyInt(Some(1)));
        // let v2 = TestFK(sea_orm::Value::Int(Some(1)));
        // println!("Result: {}", v1.eq(&v2));

        fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
            s.match_indices(p)
                .nth(n)
                .map(|(index, _)| s.split_at(index))
        }

        let a = format!("{:?}", self.0);
        let b = format!("{:?}", other.0);

        let a = split_at_nth_char(a.as_str(), '(', 1).map(|v| v.1);
        let b = split_at_nth_char(b.as_str(), '(', 1).map(|v| v.1);

        a.eq(&b)
    }
}

impl<Filter, Order> Eq for RelationKeyStruct<Filter, Order> {}

impl<Filter, Order> std::hash::Hash for RelationKeyStruct<Filter, Order> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO this is a hack

        fn split_at_nth_char(s: &str, p: char, n: usize) -> Option<(&str, &str)> {
            s.match_indices(p)
                .nth(n)
                .map(|(index, _)| s.split_at(index))
        }

        let a = format!("{:?}", self.0);
        let a = split_at_nth_char(a.as_str(), '(', 1).map(|v| v.1);

        a.hash(state)
        // TODO else do the following
        // match self.0 {
        //     sea_orm::Value::TinyInt(int) => int.unwrap().hash(state),
        //     sea_orm::Value::SmallInt(int) => int.unwrap().hash(state),
        //     sea_orm::Value::Int(int) => int.unwrap().hash(state),
        //     sea_orm::Value::BigInt(int) => int.unwrap().hash(state),
        //     sea_orm::Value::TinyUnsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::SmallUnsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::Unsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::BigUnsigned(int) => int.unwrap().hash(state),
        //     sea_orm::Value::String(str) => str.unwrap().hash(state),
        //     sea_orm::Value::Uuid(uuid) => uuid.unwrap().hash(state),
        //     _ => format!("{:?}", self.0).hash(state)
        // }
    }
}

pub async fn fetch_relation_data<Entity, Filter, Order>(
    keys: Vec<RelationKeyStruct<Option<Filter>, Option<Order>>>,
    relation: sea_orm::RelationDef,
    db: &sea_orm::DatabaseConnection,
) -> std::result::Result<
    Vec<(
        RelationKeyStruct<Option<Filter>, Option<Order>>,
        <Entity as sea_orm::EntityTrait>::Model,
    )>,
    sea_orm::error::DbErr,
>
where
    Entity: sea_orm::EntityTrait,
    <Entity::Column as FromStr>::Err: Debug,
{
    use heck::ToSnakeCase;
    use sea_orm::prelude::*;

    let keys: Vec<sea_orm::Value> = keys.into_iter().map(|key| key.0).collect();

    // TODO support multiple columns
    let to_column =
        <Entity::Column as FromStr>::from_str(relation.to_col.to_string().to_snake_case().as_str())
            .unwrap();

    let stmt = <Entity as sea_orm::EntityTrait>::find();

    let stmt =
        <sea_orm::Select<Entity> as sea_orm::QueryFilter>::filter(stmt, to_column.is_in(keys));

    let data = stmt.all(db).await?.into_iter().map(
        |model: <Entity as EntityTrait>::Model| -> (
            RelationKeyStruct<Option<Filter>, Option<Order>>,
            <Entity as EntityTrait>::Model,
        ) {
            let key = RelationKeyStruct::<Option<Filter>, Option<Order>>(
                model.get(to_column),
                None,
                None,
            );

            (key, model)
        },
    );

    Ok(data.collect())
}
