use quote::{
    __private::{TokenStream, TokenTree},
    quote,
};
use syn::{Data, DeriveInput, Fields};

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

pub fn get_fields_with_attribute(current_struct: DeriveInput) -> Vec<TokenStream> {
    let mut quotes_fields: Vec<TokenStream> = Vec::new();

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
                            quotes_fields.push(const_name);
                            const_name = quote!();
                            continue 'token_group;
                        }
                    }
                    const_name.extend(quote!(#token));
                }

                quotes_fields.push(const_name);
            }
        }

        quotes_fields.push(quote!(self.#name));
    }

    quotes_fields
}

pub fn get_fields(current_struct: DeriveInput) -> impl Iterator<Item = syn::Field> {
    let raw_fields = extract!(current_struct.data, Data::Struct(s) => s.fields, "only structs!");
    let fields = extract!(raw_fields, Fields::Named(f) => f.named, "only named fields!");

    fields.into_iter()
}
