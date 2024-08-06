extern crate proc_macro;
use core::panic;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Ident, LitInt};

#[proc_macro_derive(Codable, attributes(opcode))]
pub fn derive_codable(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let ast = Ast::new(&input);
    let impl_opcode_enum = ast.impl_codable();

    quote! {
        #impl_opcode_enum
    }
    .into()
}

#[derive(Debug, Clone)]
struct Ast<'a> {
    name: &'a Ident,
    variant_names: Vec<&'a Ident>,
    variant_opcodes: Vec<LitInt>,
    variant_tuples: Vec<Vec<String>>,
}

impl<'a> Ast<'a> {
    fn new(ast: &'a DeriveInput) -> Self {
        let name = &ast.ident;
        let data = match &ast.data {
            Data::Enum(x) => x,
            _ => panic!("expected an enum. Cannot derive a struct or union"),
        };

        let variant_names = data.variants.iter().map(|x| &x.ident).collect::<Vec<_>>();
        let variant_opcodes = data
            .variants
            .iter()
            .map(|x| {
                for attr in &x.attrs {
                    if attr.path().is_ident("opcode") {
                        let a: LitInt = attr
                            .parse_args()
                            .expect("Expect an opcode attr on each variant");
                        return a;
                    }
                }
                panic!("Should not be here. Provide a u8 opcode attr to each variant");
            })
            .collect::<Vec<_>>();
        let variant_tuples = data
            .variants
            .iter()
            .map(|x| {
                if x.fields.is_empty() {
                    Vec::<String>::new()
                } else {
                    x.fields
                        .iter()
                        .map(|y| y.ty.to_token_stream().to_string())
                        .collect::<Vec<_>>()
                }
            })
            .collect::<Vec<_>>();

        Self {
            name,
            variant_names,
            variant_opcodes,
            variant_tuples,
        }
    }

    fn impl_codable_trait(_: &Ident) -> impl ToTokens {
        quote! {
            use crate::traits::Codable;

            impl Codable for u16 {
                fn encode_op(&self) -> u16 {
                    self & 0xff
                }

                fn encode_reg1(&self, r1: u8) -> u16 {
                    self.encode_op() | ((r1 as u16) & 0xf) << 8
                }

                fn encode_reg2(&self, r1: u8, r2: u8) -> u16 {
                    self.encode_op() | ((r1 as u16) & 0xf) << 8 | ((r2 as u16) & 0xf) << 12
                }

                fn encode_arg(&self, arg: u8) -> u16 {
                    self.encode_op() | (arg as u16) << 8
                }

                fn decode_op(&self) -> u8 {
                    (self & 0xff) as u8
                }

                fn decode_reg1(&self) -> u8 {
                    ((self >> 8) as u8) & 0xf
                }

                fn decode_reg2(&self) -> (u8, u8) {
                    let r2 = (self >> 12) as u8;
                    let r1 = ((self >> 8) & 0xf) as u8;
                    (r1, r2)
                }

                fn decode_arg(&self) -> u8 {
                    (self >> 8) as u8
                }
            }
        }
    }

    fn impl_code_enum(names: &[&Ident], opcodes: &[LitInt]) -> impl ToTokens {
        quote! {
            #[derive(Debug, PartialEq, PartialOrd)]
            #[repr(u8)]
            pub enum Code {
                #( #names = #opcodes, )*
            }

            impl std::convert::TryFrom<u8> for Code {
                type Error = crate::errors::Jerror;

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    use crate::vme;
                    Ok(match value {
                        #( #opcodes => Self::#names, )*
                        _ => return Err(vme!(UnknownInstruction, "found 0x{value:02x}")),
                    })
                }
            }

            impl std::convert::TryFrom<u16> for Code {
                type Error = crate::errors::Jerror;

                fn try_from(value: u16) -> Result<Self, Self::Error> {
                    use crate::vme;
                    Ok(match (value as u8) {
                        #( #opcodes => Self::#names, )*
                        _ => return Err(vme!(UnknownInstruction, "found 0x{value:02x}")),
                    })
                }
            }

            impl std::str::FromStr for Code {
                type Err = crate::errors::Jerror;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    use crate::asme;

                    Ok(match s {
                        #( stringify!(#names) => Self::#names, )*
                        _ => return Err(asme!(ParseStr, "found {s}")),

                    })
                }
            }
        }
    }

    fn impl_encode(name: &Ident, names: &[&Ident], types: &[Vec<String>]) -> impl ToTokens {
        let encoded = types
            .iter()
            .zip(names.iter())
            .map(|(encode, variant)| {
                let encode = encode.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                if encode.is_empty() {
                    return quote! {
                        #name::#variant => Code::#variant as u16,
                    };
                }
                match encode[..] {
                    ["u8"] => quote! {
                        #name::#variant(u) => Code::#variant as u16 | ((*u as u16) << 8),
                    },
                    ["Register"] => quote! {
                        #name::#variant(r) => Code::#variant as u16 | ((*r as u16) << 8),
                    },
                    ["Register", "Register"] => quote! {
                        #name::#variant(r1,r2) => Code::#variant as u16 | ((*r1 as u16) << 8) | ((*r2 as u16) << 12),
                    },
                    _ => panic!("Dunno how to handle these"),
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl #name {
                pub fn encode(&self) -> u16 {
                    match self {
                        #( #encoded )*
                    }

                }
            }
        }
    }

    fn impl_decode(name: &Ident, names: &[&Ident], types: &[Vec<String>]) -> impl ToTokens {
        let decoded = types
            .iter()
            .zip(names.iter())
            .map(|(decode, variant)| {
                let decode = decode.iter().map(|x| x.as_str()).collect::<Vec<_>>();
                if decode.is_empty() {
                    return quote! {
                        Code::#variant => #name::#variant,
                    };
                }
                match decode[..] {
                    ["u8"] => quote! {
                        Code::#variant => {
                            let arg = value.decode_arg();
                            #name::#variant(arg)
                        },

                    },
                    ["Register"] => quote! {
                        Code::#variant => {
                            let r = Register::try_from(value.decode_reg1())?;
                            #name::#variant(r)
                        },
                    },
                    ["Register", "Register"] => quote! {
                        Code::#variant => {
                            let (r1, r2) = value.decode_reg2();
                            let r1 = Register::try_from(r1)?;
                            let r2 = Register::try_from(r2)?;
                            #name::#variant(r1, r2)
                        },
                    },
                    _ => panic!("Dunno how to decode"),
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl std::convert::TryFrom<u16> for #name {
                type Error = crate::errors::Jerror;

                fn try_from(value: u16) -> Result<Self, Self::Error> {
                    use crate::vme;
                    Ok(match Code::try_from(value)? {
                        #( #decoded )*
                        _ => return Err(vme!(UnknownInstruction, "found 0x{value:02x}")),
                    })
                }
            }
        }
    }

    fn impl_codable(self) -> impl ToTokens {
        let Ast {
            name,
            variant_names,
            variant_opcodes,
            variant_tuples,
        } = self;
        println!("{variant_tuples:#?}");

        let code_enum = Self::impl_code_enum(&variant_names, &variant_opcodes);
        let codable_trait = Self::impl_codable_trait(name);
        let codable_encode = Self::impl_encode(name, &variant_names, &variant_tuples);
        let codable_decode = Self::impl_decode(name, &variant_names, &variant_tuples);

        quote! {
            #code_enum

            #codable_trait

            #codable_encode
            #codable_decode
        }
    }
}
