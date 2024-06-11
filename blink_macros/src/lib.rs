use proc_macro::TokenStream;
use proc_macro2::token_stream;
use quote::quote;
use syn;

#[proc_macro_derive(Packet)]
pub fn packet_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_packet_macro(&ast)
}

fn impl_packet_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Packet can only be derived for structs"),
    };
    let encoded_fields = match fields {
        syn::Fields::Named(named_fields) => {
            let field_encoders = named_fields.named.iter().map(|field| {
                let field_name = field.ident.clone().unwrap();
                let field_type = field.ty.clone();
                match field_type {
                    syn::Type::Array(_) => todo!(),
                    syn::Type::Path(type_path) => {
                        let mut result = quote! {};
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "String" {
                                result = quote! {
                                    buffer.push(#field_name.as_bytes().reverse());
                                };
                            } else if ident == "i8" || ident == "i16" || ident == "i32" || ident == "i64" || ident == "i128" ||
                                ident == "u8" || ident == "u16" || ident == "u32" || ident == "u64" || ident == "u128" || ident == "f32" || ident == "f64" || ident == "usize" || ident == "iusize" {
                                result = quote! {
                                    buffer.push(#field_name.to_be_bytes());
                                }
                            }
                        }
                        result
                    }
                    syn::Type::Slice(_) => todo!(),
                    syn::Type::Tuple(_) => todo!(),
                    _ => todo!(),
                }
            });
            quote! {
                #(#field_encoders)*
            }
        }
        _ => panic!("Packet can only be derived for structs with named fields!"),
    };

    let decoded_fields = match fields {
        syn::Fields::Named(named_fields) => {
            
        }
        _ => panic!("Packet can only be derived for structs with named fields!"),
    };
    let gen = quote! {
        impl Packet for #name where #name : Sized {
            fn encode(&self) -> Vec<u8> {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<#name>());
                #encoded_fields
                buffer
            }

            fn decode(buffer: &[u8]) -> Self {

                Self {}
            }
        }
    };
    gen.into()
}
