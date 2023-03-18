//! This module provides the implementation for the [`macro@convert`](super::convert) macro.


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


/// This function implements the [`macro@convert`](super::convert) macro.
pub fn convert(input: TokenStream) -> TokenStream {
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
            type Error = serde_json::Error;
        
            fn try_into(self) -> Result<Vec<u8>, serde_json::Error> {
                serde_json::to_vec(&self)
            }
        }
        impl #generics TryInto<String> for #struct_name_ident #generics #where_clause {
            type Error = serde_json::Error;
        
            fn try_into(self) -> Result<String, serde_json::Error> {
                serde_json::to_string(&self)
            }
        }
        impl #generics TryInto<serde_json::Value> for #struct_name_ident #generics #where_clause {
            type Error = serde_json::Error;
        
            fn try_into(self) -> Result<serde_json::Value, serde_json::Error> {
                serde_json::to_value(self)
            }
        }
        impl #generics TryFrom<Vec<u8>> for #struct_name_ident #generics #where_clause {
            type Error = serde_json::Error;
        
            fn try_from(value: Vec<u8>) -> Result<Self, serde_json::Error> {
                // strip the bytes_string from trailing characters
                let mut striped_value: Vec<u8> = vec![];
                for element in value {
                    if element > 0 {
                        striped_value.push(element);
                    }
                }
                
                serde_json::from_slice(&striped_value)
            }
        }
        impl #generics TryFrom<String> for #struct_name_ident #generics #where_clause {
            type Error = serde_json::Error;
        
            fn try_from(value: String) -> Result<Self, serde_json::Error> {
                serde_json::from_str(&value)
            }
        }
        impl #generics TryFrom<serde_json::Value> for #struct_name_ident #generics #where_clause {
            type Error = serde_json::Error;
        
            fn try_from(value: serde_json::Value) -> Result<Self, serde_json::Error> {
                serde_json::from_value(value)
            }
        }
    }
    .into()
}