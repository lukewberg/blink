use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn;
use syn::Type;

#[proc_macro_derive(BedrockPacket)]
pub fn packet_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_bedrock_packet_macro(&ast)
}

fn impl_bedrock_packet_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
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
                    syn::Type::Array(_) => panic!(),
                    syn::Type::Path(type_path) => {
                        let mut result = quote! {};
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "String" {
                                let var_name_bytes = format_ident!("{field_name}_bytes");
                                result = quote! {
                                    let mut #var_name_bytes = self.#field_name.into_bytes();
                                    #var_name_bytes.reverse();
                                    buffer.extend_from_slice(&#var_name_bytes[..]);
                                };
                            } else if ident == "i8"
                                || ident == "i16"
                                || ident == "i32"
                                || ident == "i64"
                                || ident == "i128"
                                || ident == "u8"
                                || ident == "u16"
                                || ident == "u32"
                                || ident == "u64"
                                || ident == "u128"
                                || ident == "f32"
                                || ident == "f64"
                                || ident == "usize"
                                || ident == "iusize"
                            {
                                result = quote! {
                                    buffer.extend_from_slice(&self.#field_name.to_be_bytes());
                                    // buffer.write_#ident::<BigEndian>().unwarap();
                                }
                            }
                        }
                        result
                    }
                    _ => panic!(),
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
            let mut field_names: Vec<Ident> = Vec::new();
            let field_decoders = named_fields.named.iter().map(|field| {
                let field_name = field.ident.clone().unwrap();
                let field_type = field.ty.clone();
                match field_type {
                    Type::Array(_) => panic!("Array!"),
                    Type::Path(type_path) => {
                        let mut result = quote! {};
                        if let Some(ident) = type_path.path.get_ident() {
                            if ident == "String" {
                                result = quote! {
                                    let (mut split_buff, buffer) = buffer.split_at(std::mem::size_of::<#type_path>());
                                    let mut split_buff_vec = split_buff.to_vec();
                                    split_buff_vec.reverse();
                                    let #field_name = String::from_utf8(split_buff_vec)?;
                                };
                            } else {
                                let read_method = match ident.to_string().as_str() {
                                    "i8" => quote! { read_i8 },
                                    "i16" => quote! {read_i16},
                                    "i32" => quote! {read_i32},
                                    "i64" => quote! {read_i64},
                                    "i128" => quote! {read_i128},
                                    "u8" => quote! {read_u8},
                                    "u16" => quote! {read_u16},
                                    "u32" => quote! {read_u32},
                                    "u64" => quote! {read_u64},
                                    "u128" => quote! {read_u128},
                                    "f32" => quote! {read_f32},
                                    "f64" => quote! {read_f64},
                                    "usize" => quote! {read_usize},
                                    "iusize" => quote! {read_iusize},
                                    other => panic!("Unsupported primitive! {}", other),
                                };
                                result = quote! {
                                    let #field_name = BigEndian::#read_method(buffer);
                                };
                            }
                            field_names.push(field_name.clone());
                        }
                        result
                    }
                    Type::Slice(_) => panic!("Slice!"),
                    Type::Tuple(_) => panic!("Tuple!"),
                    _ => panic!("Something else happened!"),
                }
            });
            quote! {
                #(#field_decoders)*
                Ok(Self {
                    #(#field_names),*
                })
            }
        }
        _ => panic!("Packet can only be derived for structs with named fields!"),
    };
    let gen = quote! {
        impl Packet for #name where #name : Sized {
            fn encode(self) -> Vec<u8> {
                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<#name>());
                #encoded_fields
                buffer
            }

            fn decode(buffer: &mut &[u8]) -> Result<Self, SerdeError> {
                use byteorder::{ByteOrder, BigEndian};
                #decoded_fields
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(JavaPacket)]
pub fn java_packet_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_java_packet_macro(&ast)
}

fn impl_java_packet_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    impl_bedrock_packet_macro(ast)
}
