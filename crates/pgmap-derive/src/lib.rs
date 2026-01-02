extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

/// Derive macro that implements `FromRow` for structs
///
/// Generates code to convert a tokio_postgres::Row into the struct
/// by matching column names with field names.
///
/// # Attributes
/// - `#[from_row(default)]` - Use Default::default() if column is missing
/// - `#[from_row(json)]` - Deserialize field from JSON column
/// - `#[serde(rename = "...")]` - Map field to different column name
///
/// # Example
/// ```ignore
/// #[derive(FromRow, Deserialize)]
/// struct User {
///     id: i32,
///     name: String,
///     #[from_row(default)]
///     email: Option<String>,
///     #[from_row(json)]
///     metadata: serde_json::Value,
/// }
/// ```
#[proc_macro_derive(FromRow, attributes(from_row))]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => {
                return syn::Error::new_spanned(name, "FromRow only supports named fields")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "FromRow only supports structs")
                .to_compile_error()
                .into();
        }
    };

    let mut field_assignments = Vec::new();

    for field in &fields.named {
        let field_name = &field.ident;
        let mut column_name = field_name.as_ref().unwrap().to_string();

        // Check for #[serde(rename = "...")] attribute
        for attr in &field.attrs {
            if attr.path().is_ident("serde") {
                if let Meta::List(meta_list) = &attr.meta {
                    let tokens_str = meta_list.tokens.to_string();
                    // Simple string extraction for rename = "..."
                    if let Some(start) = tokens_str.find("rename = \"") {
                        let rest = &tokens_str[start + 10..];
                        if let Some(end) = rest.find('"') {
                            column_name = rest[..end].to_string();
                            break;
                        }
                    }
                }
            }
        }

        // Check for #[from_row(default)] attribute
        let has_default = field.attrs.iter().any(|attr| {
            if attr.path().is_ident("from_row") {
                if let Meta::List(meta_list) = &attr.meta {
                    meta_list.tokens.to_string().contains("default")
                } else {
                    false
                }
            } else {
                false
            }
        });

        // Check for #[from_row(json)] attribute
        let has_json = field.attrs.iter().any(|attr| {
            if attr.path().is_ident("from_row") {
                if let Meta::List(meta_list) = &attr.meta {
                    meta_list.tokens.to_string().contains("json")
                } else {
                    false
                }
            } else {
                false
            }
        });

        let assignment = if has_json {
            quote! {
                #field_name: {
                    let json_val: serde_json::Value = row.try_get(#column_name)?;
                    serde_json::from_value(json_val)?
                }
            }
        } else if has_default {
            quote! {
                #field_name: row.try_get(#column_name).unwrap_or_default()
            }
        } else {
            quote! {
                #field_name: row.try_get(#column_name)?
            }
        };

        field_assignments.push(assignment);
    }

    let expanded = quote! {
        impl pgmap::FromRow for #name {
            fn from_row(row: &tokio_postgres::Row) -> Result<Self, Box<dyn std::error::Error>> {
                Ok(Self {
                    #(#field_assignments),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
