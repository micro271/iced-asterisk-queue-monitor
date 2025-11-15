use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr,Lit, Meta, parse::Parser, punctuated::Punctuated, token::Comma};


#[proc_macro_derive(ParserEvent, attributes(parser, use_parse, key, skip_with_defaut))]
pub fn parse_input_derive_macro(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    impl_parse_input_trait(ast)
}

fn impl_parse_input_trait(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;

    let fields = match &ast.data {
        Data::Struct(data_struct) => {
            data_struct.fields.iter().map(|x| {
                let attr = &x.attrs;
                let ident = x.ident.as_ref().expect("Indent not found");
                if let Some(Meta::Path(_)) = attr.iter().find(|x| x.path().is_ident("skip_with_defaut")).map(|x| &x.meta) {
                    let ty = &x.ty;
                    return quote! {
                        #ident: <#ty as Default>::default(),
                    };
                }

                if let Some(Meta::List(meta_list)) = &attr.iter().find(|x| x.path().is_ident("parser")).map(|x| &x.meta) {
                    
                    let parsed_tokens = Punctuated::<Meta, Comma>::parse_terminated.parse2(meta_list.tokens.clone()).unwrap();
                    let mut parser = false;
                    let mut key: Option<Vec<String>> = None;
                    for nested in parsed_tokens {
                        match &nested {
                            Meta::Path(path) if path.is_ident("use_parse") => {
                                parser = true;
                            },
                            Meta::NameValue(meta_name_value) => {

                                if meta_name_value.path.is_ident("key") {
                                    if let Expr::Lit(e) = &meta_name_value.value && let Lit::Str(value) = &e.lit {
                                        if let Some(key) = key.as_mut() {
                                            key.push(value.value());
                                        } else {
                                            key = Some(vec![value.value()]);
                                        }
                                    }
                                }
                            }
                            _ => { unimplemented!() }
                        }
                    }

                    let key = key.filter(|x| !x.is_empty()).expect("Key not defined");

                    if parser {
                        
                        quote! {
                            #ident: vec![#(#key),*].into_iter().find_map(|x| data.remove(x).and_then(|x| x.parse().ok())).unwrap_or_default()
                        }
                    } else {
                        let ty = &x.ty;                        
                        quote! {
                            #ident: {
                                vec![#(#key),*].into_iter().find_map(|x| data.remove(x)).map(#ty::from).unwrap_or_default()
                            }
                        }
                    }
                } else {
                    panic!("Attributte (parser) not found");
                }
                
            }).collect::<Vec<_>>()
        },
        Data::Enum(_) => panic!("Enum are not sopported"),
        Data::Union(_) => panic!("Union are not sopported"),
    };
    
    quote::quote! {
        impl crate::asterisk::event::ParserEvent for #ident {
            fn parse_from_map(mut data: std::collections::HashMap<&str, &str>) -> #ident {
                #ident {
                    #(#fields),*
                }
            }
        }
    }.into()
}