use syn::{parse_macro_input, spanned::Spanned};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, quote_spanned};
use syn;

#[proc_macro_derive(FromSql)]
pub fn from_sql_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    let data = &ast.data;

    match data {
        syn::Data::Enum(data_enum) => {
            let cases = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let name = &variant.ident;
                    let name_string = variant.ident.to_string();

                    quote_spanned!(variant.span()=> Ok(#name_string) => Ok(Self::#name),)
                })
                .collect::<TokenStream2>();

            let gen = quote! {
                impl rusqlite::types::FromSql for #name {
                    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
                        match value.as_str() {
                            #cases
                            _ => Err(rusqlite::types::FromSqlError::InvalidType),
                        }
                    }
                }
            };

            gen.into()
        }
        _ => return quote! {compile_error!("FromSql supports only enums")}.into(),
    }
}

#[proc_macro_derive(ToSql)]
pub fn to_sql_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    let data = &ast.data;

    match data {
        syn::Data::Enum(data_enum) => {
            let cases = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let name = &variant.ident;
                    let name_string = variant.ident.to_string();

                    quote_spanned!(variant.span()=> Self::#name => Ok(rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Text(
                        #name_string.as_bytes(),
                    ))),)
                })
                .collect::<TokenStream2>();

            let gen = quote! {
                impl rusqlite::types::ToSql for #name {
                    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                        match self {
                            #cases
                        }
                    }
                }
            };

            gen.into()
        }
        _ => return quote! {compile_error!("ToSql supports only enums")}.into(),
    }
}
