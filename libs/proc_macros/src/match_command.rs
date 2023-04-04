//! This module provides the code for the [`macro@MatchCommand`](super::MatchCommand) macro.

use proc_macro::{
    self,
    TokenStream,
};
use proc_macro2::Span;
use quote::{
    format_ident,
    quote,
    quote_spanned,
};
use syn::{
    parse_macro_input,
    spanned::Spanned,
    Data::Enum,
    DeriveInput,
    Error,
    Fields,
};

/// This function implements the [`macro@MatchCommand`](super::MatchCommand) macro.
pub fn match_command(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: struct_name_ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let where_clause = &generics.where_clause;

    let mut exec = quote! {};
    match data {
        Enum(command_enum) => {
            for variant in &command_enum.variants {
                let variant_name_ident = &variant.ident;
                let variant_name_string = variant_name_ident.to_string();

                // generate the names for functions and structs associated with this variant
                let mut variant_function_name =
                    format_ident!("execute_{}", variant_name_string.to_lowercase());
                variant_function_name.set_span(variant_name_ident.span());
                let mut variant_args_name = format_ident!("{}Args", variant_name_string);
                variant_args_name.set_span(variant_name_ident.span());

                // saves the parameters of this variant to a variable
                let args = if let Fields::Unnamed(_) = &variant.fields {
                    quote_spanned! {variant.span() => args}
                } else {
                    return Error::new(
                        Span::call_site(),
                        "Only variants of the following syntax are allowed: `Start(T: Convert)`",
                    )
                    .to_compile_error()
                    .into();
                };

                // generate the code for this variant
                let exec_code = quote_spanned! {variant.span()=>
                    #struct_name_ident::#variant_name_ident(#args)=> {
                        tokio::spawn(self.clone().#variant_function_name(#args.clone()));
                    },
                };

                // save all generated code snippets for this variant to the global code
                exec = quote! {
                    #exec
                    #exec_code
                };
            }
        }
        _ => {
            return Error::new(
                Span::call_site(),
                "MatchCommand is only available for enums!",
            )
            .to_compile_error()
            .into()
        }
    };

    // implement the code generated for this enum
    quote! {
        impl #generics #struct_name_ident #generics #where_clause {
            /// Execute an asynchronous function associated with the variant of a given enum. \
            /// If the client lacks the permission to execute a given command, this method will return an error of kind [`MCManageError::MissingPermission`].
            pub fn execute(&self) -> Result<(), MCManageError> {
                match self {
                    #exec
                }
                return Ok(());
            }
        }
    }
    .into()
}