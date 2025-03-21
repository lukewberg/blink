use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemEnum, Type};

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

    let field_sizes = match fields {
        syn::Fields::Named(named_fields) => {
            let result = named_fields.named.iter().map(|field| {
            let field_name = field.ident.clone().unwrap();
            let field_type = field.ty.clone();
            match field_type {
                syn::Type::Array(_) => panic!(),
                syn::Type::Path(type_path) => {
                    let mut result = quote! {};
                    if let Some(ident) = type_path.path.get_ident() {
                        match &(*(ident.to_string())) {
                            "String" => {
                                result = quote! {
                                    packet_length = packet_length + VarInt::expected_size(self.#field_name.len() as i32) + self.#field_name.len();
                                }
                            }
                            "VarInt" => {
                                result = quote! {
                                    packet_length = packet_length + VarInt::expected_size(self.#field_name.value);
                                }
                            }
                            "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32"
                            | "u64" | "u128" | "f32" | "f64" | "usize" | "iusize" => {
                                result = quote! {
                                    packet_length = packet_length + std::mem::size_of_val(&self.#field_name);
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
                #(#result)*
            }
        }
        _ => panic!("Packet can only be derived for structs with named fields!"),
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
                                    result = quote! {
                                        stream.write_string(self.#field_name)?;
                                    }
                                }
                                "VarInt" => {
                                    result = quote! {
                                        stream.write_varint(&mut self.#field_name)?;
                                    }
                                }
                                "u8" => {
                                    result = quote! {
                                        stream.write_u8(self.#field_name)?;
                                    }
                                }
                                "i8" | "i16" | "i32" | "i64" | "i128" | "u16" | "u32"
                                | "u64" | "u128" | "f32" | "f64" | "usize" | "iusize" => {
                                    let write_method = format_ident!("write_{}", ident.to_string());
                                    result = quote! {
                                        // stream.write_all(&self.#field_name.to_be_bytes());
                                        stream.#write_method::<byteorder::BigEndian>(self.#field_name)?;
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
            fn encode<R>(mut self: #name, stream: &mut R, packet_id: u8) -> Result<(), crate::types::SerdeError> where R: crate::protocol::traits::WriteMCTypesExt {
                let packet_id = crate::types::VarInt { value: packet_id as i32 }.encode();
                let mut packet_length: usize = packet_id.len();
                #field_sizes
                let expected_header_size_field_size = crate::types::VarInt::expected_size(packet_length as i32);
                packet_length = packet_length + expected_header_size_field_size;
                stream.write_all((crate::types::VarInt { value: (packet_length - expected_header_size_field_size) as i32 }.encode()).as_slice())?;
                stream.write_all(packet_id.as_slice())?;
                #encoded_fields
                Ok(())
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

#[proc_macro_attribute]
pub fn protocol_handler(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse input as an enum
    let input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;
    let trait_name = Ident::new(&format!("{}Handler", enum_name), enum_name.span());

    // Parse optional arguments (if necessary)
    // let _args = parse_macro_input!(args with Punctuated::<MetaList, Comma>::parse_terminated);
    let arg_ident = parse_macro_input!(args as Ident);
    // Generate trait methods
    let generated_protocol_handlers = input.variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;

        // Ensure the variant has exactly two fields
        let mut fields_iter = variant.fields.iter();
        let variant_value_in = fields_iter.next()?;
        let variant_value_out = fields_iter.next()?;

        if fields_iter.next().is_some() {
            return None; // Skip variants that don't have exactly two fields
        }

        let fn_name = Ident::new(&format!("handle_{}", variant_name.to_string().to_lowercase()), enum_name.span());

        Some(quote! {
            fn #fn_name(packet: &#variant_value_in, stream: &mut TcpStream, client: &mut #arg_ident) -> Result<crate::types::StreamStatus, crate::types::SerdeError>;
        })
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
