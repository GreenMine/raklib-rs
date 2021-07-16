use std::borrow::Borrow;

use quote::{__private::TokenTree, quote};

use proc_macro::TokenStream;
use syn::{Attribute, Data, DeriveInput, Fields};

macro_rules! extract {
    ($expression:expr, $pattern:pat => $getter:expr) => {
        extract!($expression, $pattern => $getter, "failed to extract")
    };
    ($expression:expr, $pattern:pat => $getter:expr, $err:literal) => {
        match $expression {
            $pattern => $getter,
            _ => unreachable!($err),
        }
    };
}

#[proc_macro_derive(PacketEncode, attributes(const_field))]
pub fn packet_encode(item: TokenStream) -> TokenStream {
    let parsed: DeriveInput = syn::parse(item).unwrap();

    let struct_name = parsed.ident;

    let raw_fields = extract!(parsed.data, Data::Struct(s) => s.fields, "only structs!");
    let fields = extract!(raw_fields, Fields::Named(f) => f.named, "only named fields!");

    let mut quotes_fields = quote!();

    for f in fields {
        let name = f.ident.as_ref().unwrap();

        if f.attrs.len() > 0 {
            let attribute = &f.attrs[0]; //FIXME: get only first attribute

            let token = attribute.tokens.clone().into_iter().nth(0).unwrap();

            let group_stream =
                extract!(token, TokenTree::Group(g) => g.stream(), "only group provided!");

            let mut const_name = quote!();
            'token_group: for token in group_stream {
                if let TokenTree::Punct(ident) = &token {
                    if ident.as_char() == ',' {
                        quotes_fields.extend(quote!(
                            bstream.add(#const_name);
                        ));
                        const_name = quote!();
                        continue 'token_group;
                    }
                }
                const_name.extend(quote!(#token));
            }

            quotes_fields.extend(quote!(
                bstream.add(#const_name);
            ));
        }

        quotes_fields.extend(quote!(
            bstream.add(self.#name);
        ));
    }

    let expanded = quote! {
        impl raklib_std::packet::PacketEncode for #struct_name {
            fn encode_payload(&self, bstream: &mut raklib_std::utils::BinaryStream) {
                #quotes_fields
            }
        }
    };

    println!("{}", expanded.to_string());
    TokenStream::from(expanded)
}
