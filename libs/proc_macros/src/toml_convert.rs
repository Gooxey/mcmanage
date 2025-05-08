//! This module provides the implementation for the [`macro@toml_convert`](super::toml_convert) macro.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput,
};

/// This function implements the [`macro@toml_convert`](super::toml_convert) macro.
pub fn toml_convert(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: struct_name_ident,
        data: _,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let where_clause = &generics.where_clause;

    // implement the code generated for this enum
    quote! {
        impl #generics TryInto<Vec<u8>> for #struct_name_ident #generics #where_clause {
            type Error = MCManageError;

            fn try_into(self) -> Result<Vec<u8>, MCManageError> {
                Ok(toml::to_string(&self)?.as_bytes().to_vec())
            }
        }
        impl #generics TryInto<String> for #struct_name_ident #generics #where_clause {
            type Error = MCManageError;

            fn try_into(self) -> Result<String, MCManageError> {
                Ok(toml::to_string(&self)?)
            }
        }
        impl #generics TryInto<toml::Value> for #struct_name_ident #generics #where_clause {
            type Error = MCManageError;

            fn try_into(self) -> Result<toml::Value, MCManageError> {
                Ok(toml::Value::try_from(self)?)
            }
        }
        impl #generics TryFrom<Vec<u8>> for #struct_name_ident #generics #where_clause {
            type Error = MCManageError;

            fn try_from(value: Vec<u8>) -> Result<Self, MCManageError> {
                // strip the bytes_string from trailing characters
                let mut striped_value: Vec<u8> = vec![];
                for element in value {
                    if element > 0 {
                        striped_value.push(element);
                    }
                }

                Ok(toml::from_str(std::str::from_utf8(&striped_value)?)?)
            }
        }
        impl #generics TryFrom<String> for #struct_name_ident #generics #where_clause {
            type Error = MCManageError;

            fn try_from(value: String) -> Result<Self, MCManageError> {
                Ok(toml::from_str(&value)?)
            }
        }
        impl #generics TryFrom<toml::Value> for #struct_name_ident #generics #where_clause {
            type Error = MCManageError;

            fn try_from(value: toml::Value) -> Result<Self, MCManageError> {
                Ok(toml::Value::try_into(value)?)
            }
        }
    }
    .into()
}
