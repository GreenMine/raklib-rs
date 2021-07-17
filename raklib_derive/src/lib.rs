use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[macro_use]
mod utils;

#[proc_macro_derive(PacketEncode, attributes(const_fields))]
pub fn packet_encode(item: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse(item).unwrap();

    let struct_name = parsed.ident.clone();
    let generics = parsed.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut result_quote = quote!();
    crate::utils::get_fields_with_attribute(parsed)
        .iter()
        .for_each(|c| {
            result_quote.extend(quote!(
                bstream.add(#c);
            ))
        });

    let expanded = quote! {
        impl #impl_generics raklib_std::packet::PacketEncode for #struct_name #ty_generics #where_clause {
            fn encode_payload(&self, bstream: &mut raklib_std::utils::BinaryStream) {
                #result_quote
            }
        }
    };

    println!("{}", expanded.to_string());
    TokenStream::from(expanded)
}

#[proc_macro_derive(PacketDecode)]
pub fn packet_decode(item: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse(item).unwrap();

    let struct_name = parsed.ident.clone();
    let generics = parsed.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut result_quote = quote!();
    crate::utils::get_fields(parsed)
        .filter_map(|f| f.ident)
        .for_each(|c| {
            result_quote.extend(quote!(
                #c: bstream.read(),
            ))
        });

    let expanded = quote! {
        impl #impl_generics raklib_std::packet::PacketDecode for #struct_name #ty_generics #where_clause {
            fn decode(bstream: &mut raklib_std::utils::BinaryStream) -> #struct_name #ty_generics {
                #struct_name {
                    #result_quote
                }
            }
        }
    };

    println!("{}", expanded.to_string());
    TokenStream::from(expanded)
}
