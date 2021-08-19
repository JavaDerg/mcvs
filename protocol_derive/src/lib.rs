#![deny(unsafe_code)]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

macro_rules! error {
    ($msg:literal) => {{
        return quote! {
            compile_error!($msg);
        }
        .into();
    }};
}

#[proc_macro_derive(Json)]
pub fn json_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    (quote! {
        impl crate::codec::Transcodeable for #ident {
            fn encode<B: bytes::BufMut>(&self, buf: B) -> std::result::Result<(), crate::codec::EncodeError> {
                crate::types::JsonRef::<#ident, { crate::types::MAX_CHARS_STR }>(std::borrow::Cow::Borrowed(self)).encode(buf)
            }

            fn decode<B: bytes::Buf>(buf: B) -> std::result::Result<Self, crate::codec::DecodeError> {
                std::result::Result::Ok(crate::types::JsonN::<#ident, { crate::types::MAX_CHARS_STR }>::decode(buf)?.0)
            }
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let args = parse_macro_input!(args as AttributeArgs);

    let data = if let Data::Struct(data) = &input.data {
        data
    } else {
        error!("Deriving Packet is only allowed on structs");
    };

    let punct = match &data.fields {
        Fields::Named(named) => Some(&named.named),
        Fields::Unnamed(unnamed) => Some(&unnamed.unnamed),
        Fields::Unit => None,
    };

    let mut pid = None;

    for meta in args.as_slice() {
        if let NestedMeta::Meta(Meta::NameValue(name)) = meta {
            if let Some(pseg) = name.path.segments.iter().next() {
                if pseg.ident != "id" {
                    continue;
                }
            } else {
                error!("????");
            }
            if let Lit::Int(num) = &name.lit {
                if pid.is_some() {
                    error!("id can not be set twice");
                }
                pid = Some(num.base10_parse::<i32>().unwrap());
            } else {
                error!("id must have an integer as value");
            }
        }
    }

    let pid = match pid {
        Some(id) => id,
        None => error!("packet id not defined"),
    };

    let struct_name = &input.ident;

    let mut encode = Vec::new();
    let mut decode = Vec::new();
    let mut decode_set = Vec::new();
    let mut size = Vec::new();

    if let Some(punct) = punct {
        for (id, field) in (0u32..).zip(punct.clone()) {
            let name = field
                .ident
                .map(|ident| quote!(#ident))
                .unwrap_or_else(|| quote!(#id));
            let local_name = format_ident!("_{}", name.to_string());

            encode.push(quote! {
                self. #name .encode(&mut buf)?;
            });
            decode.push(quote! {
                let #local_name = Transcodeable::decode(&mut buf)?;
            });
            decode_set.push(quote! {
                #name: #local_name
            });
            size.push(quote! {
                size += self. #name . size_hint()?;
            });
        }
    }

    (quote! {
        #input

        impl crate::codec::Packet for #struct_name {
            fn id() -> i32 {
                #pid
            }
        }

        impl crate::codec::Transcodeable for #struct_name  {
            fn encode<B: bytes::BufMut>(&self, mut buf: B) -> std::result::Result<(), crate::codec::EncodeError> {
                use crate::codec::Transcodeable;
                #(#encode)*
                std::result::Result::Ok(())
            }

            fn decode<B: bytes::Buf>(mut buf: B) -> std::result::Result<Self, crate::codec::DecodeError> {
                use crate::codec::Transcodeable;
                #(#decode)*
                std::result::Result::Ok(Self { #(#decode_set),* })
            }

            fn size_hint(&self) -> std::option::Option<usize> {
                let mut size = 0usize;
                #(#size)*
                std::option::Option::Some(size)
            }
        }
    })
    .into()
}
