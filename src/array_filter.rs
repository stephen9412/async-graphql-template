// Derived from Seaography (github.com/SeaQL/seaography)
// Modifications Copyright (c) 2025 Stephen J. Li

/// 整數數組過濾器
/// 
/// 用於支持對 Vec<i32> 類型的數據進行高級過濾操作
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct IntArrayFilter {
    /// 完全匹配數組
    pub eq: Option<Vec<i32>>,
    /// 不等於數組
    pub ne: Option<Vec<i32>>,
    pub is_null: Option<bool>,
    
    /// 包含所有指定元素（PostgreSQL @>）
    pub contains: Option<Vec<i32>>,
    /// 包含任意指定元素（PostgreSQL &&）
    pub contains_any: Option<Vec<i32>>,
}

/// 字符串數組過濾器
/// 
/// 用於支持對 Vec<String> 類型的數據進行高級過濾操作
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct StringArrayFilter {
    /// 完全匹配數組
    pub eq: Option<Vec<String>>,
    /// 不等於數組
    pub ne: Option<Vec<String>>,
    pub is_null: Option<bool>,
    
    /// 包含所有指定元素（PostgreSQL @>）
    pub contains: Option<Vec<String>>,
    /// 包含任意指定元素（PostgreSQL &&）
    pub contains_any: Option<Vec<String>>,
}

/// 小型整數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct TinyIntArrayFilter {
    pub eq: Option<Vec<i8>>,
    pub ne: Option<Vec<i8>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<i8>>,
    pub contains_any: Option<Vec<i8>>,
}

/// 短整數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct SmallIntArrayFilter {
    pub eq: Option<Vec<i16>>,
    pub ne: Option<Vec<i16>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<i16>>,
    pub contains_any: Option<Vec<i16>>,
}

/// 大整數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct BigIntArrayFilter {
    pub eq: Option<Vec<i64>>,
    pub ne: Option<Vec<i64>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<i64>>,
    pub contains_any: Option<Vec<i64>>,
}

/// 無符號整數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct UnsignedArrayFilter {
    pub eq: Option<Vec<u32>>,
    pub ne: Option<Vec<u32>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<u32>>,
    pub contains_any: Option<Vec<u32>>,
}

/// 小型無符號整數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct TinyUnsignedArrayFilter {
    pub eq: Option<Vec<u8>>,
    pub ne: Option<Vec<u8>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<u8>>,
    pub contains_any: Option<Vec<u8>>,
}

/// 短無符號整數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct SmallUnsignedArrayFilter {
    pub eq: Option<Vec<u16>>,
    pub ne: Option<Vec<u16>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<u16>>,
    pub contains_any: Option<Vec<u16>>,
}

/// 大無符號整數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct BigUnsignedArrayFilter {
    pub eq: Option<Vec<u64>>,
    pub ne: Option<Vec<u64>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<u64>>,
    pub contains_any: Option<Vec<u64>>,
}

/// 浮點數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct FloatArrayFilter {
    pub eq: Option<Vec<f32>>,
    pub ne: Option<Vec<f32>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<f32>>,
    pub contains_any: Option<Vec<f32>>,
}

/// 雙精度浮點數數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct DoubleArrayFilter {
    pub eq: Option<Vec<f64>>,
    pub ne: Option<Vec<f64>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<f64>>,
    pub contains_any: Option<Vec<f64>>,
}

/// 布爾值數組過濾器
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct BooleanArrayFilter {
    pub eq: Option<Vec<bool>>,
    pub ne: Option<Vec<bool>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<bool>>,
    pub contains_any: Option<Vec<bool>>,
}

#[cfg(feature = "with-uuid")]
#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct UuidArrayFilter {
    pub eq: Option<Vec<sea_orm::prelude::Uuid>>,
    pub ne: Option<Vec<sea_orm::prelude::Uuid>>,
    pub is_null: Option<bool>,
    pub contains: Option<Vec<sea_orm::prelude::Uuid>>,
    pub contains_any: Option<Vec<sea_orm::prelude::Uuid>>,
}
