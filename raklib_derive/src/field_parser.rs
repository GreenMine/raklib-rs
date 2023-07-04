use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub enum StructField {
    Basic(Ident),
    Const(TokenStream),
}

pub fn get_fields_with_attribute(current_struct: DeriveInput) -> Vec<StructField> {
    let mut quotes_fields: Vec<StructField> = Vec::new();

    for f in get_fields(current_struct) {
        let name = f.ident.unwrap();

        if f.attrs.len() > 0 {
            for attribute in f.attrs {
                let token = attribute.tokens.into_iter().nth(0).unwrap();

                let group_stream =
                    extract!(token, TokenTree::Group(g) => g.stream(), "only group provided!");

                let mut const_name = quote!();
                'token_group: for token in group_stream {
                    if let TokenTree::Punct(ident) = &token {
                        if ident.as_char() == ',' {
                            quotes_fields.push(StructField::Const(const_name));
                            const_name = quote!();
                            continue 'token_group;
                        }
                    }
                    const_name.extend(quote!(#token));
                }
                quotes_fields.push(StructField::Const(const_name));
            }
        }

        quotes_fields.push(StructField::Basic(name));
    }

    quotes_fields
}

pub fn get_fields(current_struct: DeriveInput) -> impl Iterator<Item = syn::Field> {
    let raw_fields = extract!(current_struct.data, Data::Struct(s) => s.fields, "only structs!");
    let fields = extract!(raw_fields, Fields::Named(f) => f.named, "only named fields!"); //FIXME: unit structs

    fields.into_iter()
}
