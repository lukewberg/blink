use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemEnum, LitStr, Meta, MetaList, Type};
use syn::punctuated::Punctuated;

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
                            match &(*(ident.to_string())) {
                                "String" => {
                                    // Start by encoding the string length as a VarInt
                                    ()
                                }
                                "VarInt" => {
                                    result = quote! {
                                        buffer.append(&mut self.#field_name.encode());
                                    }
                                }
                                "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32"
                                | "u64" | "u128" | "f32" | "f64" | "usize" | "iusize" => {
                                    result = quote! {
                                        buffer.extend_from_slice(&self.#field_name.to_be_bytes());
                                        // buffer.write_#ident::<BigEndian>().unwarap();
                                    }
                                }
                                _ => (),
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
                                    // Decoding string
                                    // First, parse the length of the string (VarInt)
                                    // let length = VarInt::decode(buffer).expect("Unable to parse string length!");
                                    // Allocate space
                                    // let mut string_buf: Vec<u8> = vec![0u8; *length as usize];
                                    // buffer
                                    //     .read_exact(&mut string_buf)
                                    //     .expect("Unable to read string bytes!");
                                    // let #field_name = String::from_utf8(string_buf)?;
                                    let #field_name = buffer.read_string()?;
                                };
                            } else {
                                let read_method = match ident.to_string().as_str() {
                                    "i8" => quote! { read_i8 },
                                    "i16" => quote! {read_i16::<byteorder::BigEndian>},
                                    "i32" => quote! {read_i32::<byteorder::BigEndian>},
                                    "i64" => quote! {read_i64::<byteorder::BigEndian>},
                                    "i128" => quote! {read_i128::<byteorder::BigEndian>},
                                    "u8" => quote! {read_u8},
                                    "u16" => quote! {read_u16::<byteorder::BigEndian>},
                                    "u32" => quote! {read_u32::<byteorder::BigEndian>},
                                    "u64" => quote! {read_u64::<byteorder::BigEndian>},
                                    "u128" => quote! {read_u128::<byteorder::BigEndian>},
                                    "f32" => quote! {read_f32::<byteorder::BigEndian>},
                                    "f64" => quote! {read_f64::<byteorder::BigEndian>},
                                    "usize" => quote! {read_usize::<byteorder::BigEndian>},
                                    "iusize" => quote! {read_iusize::<byteorder::BigEndian>},
                                    "VarInt" => quote! {read_varint},
                                    other => panic!("Unsupported primitive! {}", other),
                                };
                                result = quote! {
                                    // decoding number primative
                                    let #field_name = buffer.#read_method()?;
                                };
                            }
                            field_names.push(field_name);
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
        impl crate::traits::NetworkPacket for #name where #name : Sized {
            fn encode(mut self: #name) -> Vec<u8> {

                let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<#name>());
                #encoded_fields
                buffer
            }

            fn decode<R>(buffer: &mut R) -> Result<Self, crate::types::SerdeError> where R: crate::protocol::traits::ReadMCTypesExt {
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


#[derive(Default)]
struct ProtocolHandlerArgs {
    test: Option<LitStr>,
}

#[proc_macro_attribute]
pub fn protocol_handler(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;
    let trait_name = Ident::new(&format!("{}Handler", enum_name.to_string()), enum_name.span());

    // let args = syn::parse_macro_input!(attr with Punctuated::<MetaList, syn::Token![,]>::parse_terminated);
    let args = syn::parse_macro_input!(args with Punctuated::<MetaList, syn::Token![,]>::parse_terminated);
    

    let generated_protocol_handlers = &input.variants.iter_mut().map(|variant| {
        let variant_name = &variant.ident;
        let variant_value_iterator = &mut variant.fields.iter();
        let variant_value_in = &variant_value_iterator.next().unwrap().ty;
        let variant_value_out = &variant_value_iterator.next().unwrap().ty;

        // Snake-case function name in the format of handle_#variant_name
        let fn_name = Ident::new(&format!("handle_{}", variant_name.to_string().to_lowercase()), enum_name.span());

        quote! {
            fn #fn_name<T: Sized>(packet: &#variant_value_in, state: &mut T) -> #variant_value_out;
        }
    }).collect::<Vec<_>>();

    let expanded = quote! {
        // Output the original enum
        #input

        // Generate a trait that can be manually implemented
        pub trait #trait_name {
            #(#generated_protocol_handlers)*
        }
    };

    proc_macro::TokenStream::from(expanded)
}