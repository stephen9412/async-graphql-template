// Derived from Seaography (github.com/SeaQL/seaography)
// Modifications Copyright (c) 2025 Stephen J. Li

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct SeaOrm {
    table_name: Option<syn::Lit>,
}

pub type IdentTypeTuple = (syn::Ident, syn::Type, bool);

// TODO skip ignored fields
pub fn filter_fn(item: syn::DataStruct, attrs: SeaOrm) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<IdentTypeTuple> = item
        .fields
        .into_iter()
        .map(|field| {
            let (ty, is_option) = remove_optional_from_type_and_get_is_option(field.ty).unwrap();
            (field.ident.unwrap(), ty, is_option)
        })
        .collect();

    let filter_struct = filter_struct(&fields, &attrs)?;

    let recursive_filter_fn = recursive_filter_fn(&fields)?;

    let order_by_struct = order_by_struct(&fields, &attrs)?;

    let order_by_fn = order_by_fn(&fields)?;

    Ok(quote! {
        #filter_struct

        #recursive_filter_fn

        #order_by_struct

        #order_by_fn
    })
}

pub fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .last()
            .map(|seg| seg.ident == "Vec")
            .unwrap_or(false)
    } else {
        false
    }
}

pub fn filter_struct(
    fields: &[IdentTypeTuple],
    attrs: &SeaOrm,
) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, ty, _)| {
            let type_literal = ty.to_token_stream().to_string();

            let default_filters = vec![
                "i8",
                "i16",
                "i32",
                "i64",
                "u8",
                "u16",
                "u32",
                "u64",
                "f32",
                "f64",
                #[cfg(feature = "with-chrono")]
                "Date",
                #[cfg(feature = "with-chrono")]
                "DateTime",
                #[cfg(feature = "with-chrono")]
                "DateTimeUtc",
                #[cfg(feature = "with-chrono")]
                "DateTimeWithTimeZone",
                #[cfg(feature = "with-decimal")]
                "Decimal",
                #[cfg(feature = "with-json")]
                "Json",
                #[cfg(feature = "with-uuid")]
                "Uuid",
                "BinaryVector",
                "bool",
            ];

            let filter_item = if is_vec_type(ty) {
                if type_literal.contains("String") {
                    quote! {
                        async_graphql_template::StringArrayFilter
                    }
                } else if type_literal.contains("i8") {
                    quote! {
                        async_graphql_template::TinyIntArrayFilter
                    }
                } else if type_literal.contains("i16") {
                    quote! {
                        async_graphql_template::SmallIntArrayFilter
                    }
                } else if type_literal.contains("i32") {
                    quote! {
                        async_graphql_template::IntArrayFilter
                    }
                } else if type_literal.contains("i64") {
                    quote! {
                        async_graphql_template::BigIntArrayFilter
                    }
                } else if type_literal.contains("u8") {
                    quote! {
                        async_graphql_template::TinyUnsignedArrayFilter
                    }
                } else if type_literal.contains("u16") {
                    quote! {
                        async_graphql_template::SmallUnsignedArrayFilter
                    }
                } else if type_literal.contains("u32") {
                    quote! {
                        async_graphql_template::UnsignedArrayFilter
                    }
                } else if type_literal.contains("u64") {
                    quote! {
                        async_graphql_template::BigUnsignedArrayFilter
                    }
                } else if type_literal.contains("f32") {
                    quote! {
                        async_graphql_template::FloatArrayFilter
                    }
                } else if type_literal.contains("f64") {
                    quote! {
                        async_graphql_template::DoubleArrayFilter
                    }
                } else if type_literal.contains("bool") {
                    quote! {
                        async_graphql_template::BooleanArrayFilter
                    }
                } else {
                    quote! {
                        async_graphql_template::IntArrayFilter
                    }
                }
            } else if default_filters.contains(&type_literal.as_str()) {
                quote! {
                    async_graphql_template::TypeFilter<#ty>
                }
            } else if &type_literal.as_str() == &"String" {
                quote! {
                    async_graphql_template::StringFilter<#ty>
                }
            } else {
                let ident = format_ident!("{}EnumFilter", type_literal);
                quote! {
                    crate::entities::sea_orm_active_enums::#ident
                }
            };

            quote! {
                #ident: Option<#filter_item>
            }
        })
        .collect();

    let entity_name = match &attrs.table_name {
        Some(syn::Lit::Str(name)) => name,
        _ => return Err(crate::error::Error::Internal("Invalid entity name".into())),
    };

    let filter_name = format!("{}Filter", entity_name.value().to_upper_camel_case());

    // TODO enable when async graphql support name_type for input objects
    // let type_name = quote!{
    //     impl async_graphql::TypeName for Filter {
    //         fn type_name() -> ::std::borrow::Cow<'static, str> {
    //             use async_graphql_template::heck::ToUpperCamelCase;

    //             let filter_name = format!("{}Filter", Entity::default().table_name().to_string().to_upper_camel_case());

    //             ::std::borrow::Cow::Owned(filter_name)
    //         }
    //     }
    // }

    Ok(quote! {
        #[derive(Debug, Clone, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct Filter {
            pub or: Option<Vec<Box<Filter>>>,
            pub and: Option<Vec<Box<Filter>>>,
            #(#fields),*
        }
    })
}

pub fn order_by_struct(
    fields: &[IdentTypeTuple],
    attrs: &SeaOrm,
) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, _, _)| {
            quote! {
                #ident: Option<async_graphql_template::OrderByEnum>
            }
        })
        .collect();

    let entity_name = match &attrs.table_name {
        Some(syn::Lit::Str(name)) => name,
        _ => return Err(crate::error::Error::Internal("Invalid entity name".into())),
    };

    let filter_name = format!("{}OrderBy", entity_name.value().to_upper_camel_case());

    Ok(quote! {
        #[derive(Debug, Clone, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct OrderBy {
            #(#fields),*
        }
    })
}

pub fn order_by_fn(fields: &[IdentTypeTuple]) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, _, _)| {
            let column = format_ident!("{}", ident.to_string().to_upper_camel_case());

            quote! {
                let stmt = if let Some(order_by) = order_by_struct.#ident {
                    match order_by {
                        async_graphql_template::OrderByEnum::Asc => stmt.order_by(Column::#column, sea_orm::query::Order::Asc),
                        async_graphql_template::OrderByEnum::Desc => stmt.order_by(Column::#column, sea_orm::query::Order::Desc),
                    }
                } else {
                    stmt
                };
            }
        })
        .collect();

    Ok(quote! {
        pub fn order_by(stmt: sea_orm::Select<Entity>, order_by_struct: Option<OrderBy>) -> sea_orm::Select<Entity> {
            use sea_orm::QueryOrder;

            if let Some(order_by_struct) = order_by_struct {
                #(#fields)*
                stmt
            } else {
                stmt
            }
        }
    })
}

pub fn recursive_filter_fn(fields: &[IdentTypeTuple]) -> Result<TokenStream, crate::error::Error> {
    let columns_filters: Vec<TokenStream> = fields
        .iter()
        .map(|(ident_proc, ident_type, _)| {

            let column_name = format_ident!("{}", ident_proc.to_string().to_snake_case());
            let column_enum_name = format_ident!("{}", ident_proc.to_string().to_upper_camel_case());

            let mut is_string = false;
            let is_vec = is_vec_type(ident_type);

            if let syn::Type::Path(syn::TypePath{ qself: _, path}) = ident_type {
                let syn::Path{ leading_colon: _, segments } = path;
                let syn::PathSegment{ ident, arguments: _ } = &segments[0];
                if ident.to_string() == "String".to_owned() {
                    is_string = true;
                }
            }

            let mut string_filter = TokenStream::new();
            let mut array_filter = TokenStream::new();

            if is_string {
                string_filter = quote!{
                    if let Some(eq_value) = &#column_name.like {
                        condition = condition.add(Column::#column_enum_name.like(eq_value))
                    }
                };
            }

            if is_vec {
                array_filter = quote!{
                    if let Some(contains) = &#column_name.contains {
                        // 使用 sea_orm 提供的 contains 函數進行數組包含檢查
                        let mut contains_condition = sea_orm::Condition::all();
                        for item in contains.iter() {
                            contains_condition = contains_condition.add(
                                sea_orm::sea_query::extension::postgres::PgExpr::contains(
                                    sea_orm::sea_query::Expr::col(Column::#column_enum_name),
                                    item.clone()
                                )
                            );
                        }
                        condition = condition.add(contains_condition);
                    }

                    if let Some(contains_any) = &#column_name.contains_any {
                        // 使用 OR 條件構建 ANY 語義
                        let mut any_condition = sea_orm::Condition::any();
                        for item in contains_any.iter() {
                            any_condition = any_condition.add(
                                sea_orm::sea_query::extension::postgres::PgExpr::contains(
                                    sea_orm::sea_query::Expr::col(Column::#column_enum_name),
                                    item.clone()
                                )
                            );
                        }
                        condition = condition.add(any_condition);
                    }
                };
            }

            // 根據類型不同選擇不同的過濾邏輯
            if is_vec {
                // 數組類型僅使用數組專用過濾條件
                quote!{
                    if let Some(#column_name) = current_filter.#column_name {
                        #array_filter

                        // 標準條件也適用於數組
                        if let Some(eq_value) = #column_name.eq {
                            condition = condition.add(Column::#column_enum_name.eq(eq_value))
                        }

                        if let Some(ne_value) = #column_name.ne {
                            condition = condition.add(Column::#column_enum_name.ne(ne_value))
                        }

                        if let Some(is_null_value) = #column_name.is_null {
                            if is_null_value {
                                condition = condition.add(Column::#column_enum_name.is_null())
                            }
                        }
                    }
                }
            } else if is_string {
                // 字符串類型使用字符串專用過濾條件
                quote!{
                    if let Some(#column_name) = current_filter.#column_name {
                        #string_filter

                        if let Some(eq_value) = #column_name.eq {
                            condition = condition.add(Column::#column_enum_name.eq(eq_value))
                        }

                        if let Some(ne_value) = #column_name.ne {
                            condition = condition.add(Column::#column_enum_name.ne(ne_value))
                        }

                        if let Some(gt_value) = #column_name.gt {
                            condition = condition.add(Column::#column_enum_name.gt(gt_value))
                        }

                        if let Some(gte_value) = #column_name.gte {
                            condition = condition.add(Column::#column_enum_name.gte(gte_value))
                        }

                        if let Some(lt_value) = #column_name.lt {
                            condition = condition.add(Column::#column_enum_name.lt(lt_value))
                        }

                        if let Some(lte_value) = #column_name.lte {
                            condition = condition.add(Column::#column_enum_name.lte(lte_value))
                        }

                        if let Some(is_in_value) = #column_name.is_in {
                            condition = condition.add(Column::#column_enum_name.is_in(is_in_value))
                        }

                        if let Some(is_not_in_value) = #column_name.is_not_in {
                            condition = condition.add(Column::#column_enum_name.is_not_in(is_not_in_value))
                        }

                        if let Some(is_null_value) = #column_name.is_null {
                            if is_null_value {
                                condition = condition.add(Column::#column_enum_name.is_null())
                            }
                        }
                    }
                }
            } else {
                // 其他一般類型使用標準過濾條件
                quote!{
                    if let Some(#column_name) = current_filter.#column_name {
                        if let Some(eq_value) = #column_name.eq {
                            condition = condition.add(Column::#column_enum_name.eq(eq_value))
                        }

                        if let Some(ne_value) = #column_name.ne {
                            condition = condition.add(Column::#column_enum_name.ne(ne_value))
                        }

                        if let Some(gt_value) = #column_name.gt {
                            condition = condition.add(Column::#column_enum_name.gt(gt_value))
                        }

                        if let Some(gte_value) = #column_name.gte {
                            condition = condition.add(Column::#column_enum_name.gte(gte_value))
                        }

                        if let Some(lt_value) = #column_name.lt {
                            condition = condition.add(Column::#column_enum_name.lt(lt_value))
                        }

                        if let Some(lte_value) = #column_name.lte {
                            condition = condition.add(Column::#column_enum_name.lte(lte_value))
                        }

                        if let Some(is_in_value) = #column_name.is_in {
                            condition = condition.add(Column::#column_enum_name.is_in(is_in_value))
                        }

                        if let Some(is_not_in_value) = #column_name.is_not_in {
                            condition = condition.add(Column::#column_enum_name.is_not_in(is_not_in_value))
                        }

                        if let Some(is_null_value) = #column_name.is_null {
                            if is_null_value {
                                condition = condition.add(Column::#column_enum_name.is_null())
                            }
                        }
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
            use sea_orm::sea_query::extension::postgres::PgExpr;
            let mut condition = sea_orm::Condition::all();

            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::any(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(or_condition);
                }

                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::all(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(and_condition);
                }

                #(#columns_filters)*
            }

            condition
        }
    })
}

fn path_is_option(path: &syn::Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Option"
}

pub fn remove_optional_from_type_and_get_is_option(
    ty: syn::Type,
) -> Result<(syn::Type, bool), crate::error::Error> {
    let mut is_option = false;

    let ty = match ty {
        syn::Type::Path(type_path)
            if type_path.qself.is_none() && path_is_option(&type_path.path) =>
        {
            is_option = path_is_option(&type_path.path);
            let type_params = &type_path.path.segments.first().unwrap().arguments;
            let generic_arg = match type_params {
                syn::PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => {
                    return Err(crate::error::Error::Internal(
                        "Cannot parse type brackets".into(),
                    ))
                }
            };
            match generic_arg {
                syn::GenericArgument::Type(ty) => ty.to_owned(),
                _ => return Err(crate::error::Error::Internal("Cannot parse type".into())),
            }
        }
        _ => ty,
    };

    Ok((ty, is_option))
}
