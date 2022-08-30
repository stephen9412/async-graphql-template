use sea_orm::prelude::*;
pub mod entities;
pub mod query_root;
pub use query_root::QueryRoot;
pub struct OrmDataloader {
    pub db: DatabaseConnection,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, async_graphql :: Enum)]
pub enum OrderByEnum {
    Asc,
    Desc,
}
pub type BinaryVector = Vec<u8>;
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(concrete(name = "StringFilter", params(String)))]
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
#[graphql(concrete(name = "DateFilter", params(Date)))]
#[graphql(concrete(name = "DateTimeFilter", params(DateTime)))]
#[graphql(concrete(name = "DateTimeUtcFilter", params(DateTimeUtc)))]
#[graphql(concrete(name = "DecimalFilter", params(Decimal)))]
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
