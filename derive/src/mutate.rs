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

pub fn mutate_fn(item: syn::DataStruct, attrs: SeaOrm) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<IdentTypeTuple> = item
        .fields
        .into_iter()
        .map(|field| {
            let (ty, is_option) = remove_optional_from_type_and_get_is_option(field.ty).unwrap();
            (field.ident.unwrap(), ty, is_option)
        })
        .collect();

    let mutant_struct = mutant_struct(&fields, &attrs)?;
    let recursive_set_fn = recursive_set_fn(&fields)?;

    Ok(quote! {
        #mutant_struct

        #recursive_set_fn
    })
}

pub fn mutant_struct(
    fields: &[IdentTypeTuple],
    attrs: &SeaOrm,
) -> Result<TokenStream, crate::error::Error> {
    let fields: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, ty, _)| {
            let type_literal = ty.to_token_stream().to_string();

            let default_filters = vec![
                "bool",
                "f32",
                "f64",
                "i8",
                "i16",
                "i32",
                "i64",
                "u8",
                "u16",
                "u32",
                "u64",
                "BinaryVector",
                "Date",
                "DateTime",
                "DateTimeUtc",
                "DateTimeWithTimeZone",
                "DeliveryTime",
                "Decimal",
                "Json",
                "String",
                "TaxType",
                "Uuid",
            ];

            let filter_item = if default_filters.contains(&type_literal.as_str()) {
                quote! {
                    #ty
                }
            } else {
                quote! {}
            };

            let ignore_fields = vec![
                "id".to_owned(),
                "user_id".to_owned(),
                "created_at".to_owned(),
            ];
            let must_have = vec![];

            let column_name = format_ident!("{}", ident.to_string().to_snake_case());
            if ignore_fields.contains(&column_name.to_string()) {
                quote! {}
            } else if must_have.contains(&column_name.to_string()) {
                quote! {
                    pub #ident: #filter_item,
                }
            } else {
                quote! {
                    pub #ident: Option<#filter_item>,
                }
            }
        })
        .collect();

    let entity_name = match &attrs.table_name {
        Some(syn::Lit::Str(name)) => name,
        _ => return Err(crate::error::Error::Internal("Invalid entity name".into())),
    };

    let filter_name = format!("{}Mutant", entity_name.value().to_upper_camel_case());

    Ok(quote! {
        #[derive(Debug, Clone, async_graphql::InputObject)]
        #[graphql(name = #filter_name)]
        pub struct Mutant {
            #(#fields)*
        }
    })
}

pub fn recursive_set_fn(fields: &[IdentTypeTuple]) -> Result<TokenStream, crate::error::Error> {
    let columns_filters: Vec<TokenStream> = fields
        .iter()
        .map(|(ident, _, is_option)| {
            let column_name = format_ident!("{}", ident.to_string().to_snake_case());
            let _column_enum_name = format_ident!("{}", ident.to_string().to_upper_camel_case());

            let ignore_fields = vec![
                "id".to_owned(),
                "user_id".to_owned(),
                "created_at".to_owned(),
            ];
            let must_have = vec![];

            if ignore_fields.contains(&column_name.to_string()) {
                quote! {}
            } else if must_have.contains(&column_name.to_string()) {
                quote! {
                    self.#column_name = Set(mutant.#column_name);
                }
            } else if is_option == &true {
                quote! {
                    if let Some(value) = mutant.#column_name {
                        self.#column_name = Set(Some(value));
                    }
                }
            } else {
                quote! {
                    if let Some(value) = mutant.#column_name {
                        self.#column_name = Set(value);
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        use sea_orm::ActiveValue::Set;
        impl ActiveModel {
            pub fn recursive_set_value(&mut self, mutant: Mutant) {
                #(#columns_filters)*
            }
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
