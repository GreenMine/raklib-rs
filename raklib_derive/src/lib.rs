use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

use field_parser::StructField;

#[macro_use]
mod utils;
mod field_parser;

#[proc_macro_derive(PacketEncode, attributes(const_fields))]
pub fn packet_encode(item: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse(item).unwrap();

    let struct_name = parsed.ident.clone();
    let generics = parsed.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut result_quote = quote!();
    crate::field_parser::get_fields_with_attribute(parsed)
        .into_iter()
        .map(|sf| match sf {
            StructField::Basic(name) => quote!(self.#name),
            StructField::Const(ts) => ts,
        })
        .for_each(|c| {
            result_quote.extend(quote!(
                bstream.add(#c);
            ))
        });

    let expanded = quote! {
        impl #impl_generics raklib_std::packet::PacketEncode for #struct_name #ty_generics #where_clause {
            fn encode_payload(&self, bstream: &mut raklib_std::stream::BinaryStream) {
                #result_quote
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(PacketDecode, attributes(const_fields))]
pub fn packet_decode(item: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse(item).unwrap();

    let struct_name = parsed.ident.clone();
    let generics = parsed.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut result_quote = quote!();
    let mut names: Vec<proc_macro2::TokenStream> = Vec::new();
    crate::field_parser::get_fields_with_attribute(parsed)
        .into_iter()
        .map(|sf| match sf {
            StructField::Basic(n) => {
                names.push(quote!(#n));
                quote!(
                    let #n = bstream.read()?;
                )
            }
            StructField::Const(_) => {
                unimplemented!("consts parse inside PacketDecode. Later try to use const-eval",)
            }
        })
        .for_each(|c| result_quote.extend(c));

    let expanded = quote! {
        impl #impl_generics raklib_std::packet::PacketDecode for #struct_name #ty_generics #where_clause {
            fn decode(bstream: &mut raklib_std::stream::BinaryStream) -> raklib_std::stream::Result<#struct_name #ty_generics> {
                #result_quote
                // assert_eq!(bstream.p, bstream.data.len());
                Ok(Self { #(#names), * })
            }
        }
    };

    TokenStream::from(expanded)
}
