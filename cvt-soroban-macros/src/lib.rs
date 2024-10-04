use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use darling::{FromMeta, ast::NestedMeta};
use quote::{quote, format_ident};
use syn::{
    parse_macro_input,
    parse_str,
    Data::{self, Struct, Union, Enum},
    DeriveInput,
    Field,
    Ident,
    ItemStruct,
    Member,
    Fields::{self, Named, Unnamed},
    FieldsNamed,
    FieldsUnnamed,
    Path,
    FnArg,
    Type,
    TypeReference,
    TypePath,
};

fn default_crate_path() -> Path {
    parse_str("soroban_sdk").unwrap()
}

#[derive(Debug, FromMeta)]
struct ContractClientArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    name: String,
    #[darling(default)]
    impl_only: bool,
}

fn non_env_inputs(f: &syn::TraitItemFn) -> (Vec<Ident>, Vec<FnArg>) {
    f.sig
        .inputs
        .iter()
        .skip(if env_input(f) { 1 } else { 0 })
        .map(|t| {
            match t {
                FnArg::Typed(ref pat_type) =>
                    match *pat_type.pat.clone() {
                        syn::Pat::Ident(x) => {
                            match *pat_type.ty.clone() {
                                Type::Reference(_) =>
                                    (x.ident, t.clone()),

                                _ => {
                                    (x.ident, FnArg::Typed(syn::PatType {
                                        attrs: pat_type.attrs.clone(),
                                        pat: pat_type.pat.clone(),
                                        colon_token: pat_type.colon_token,
                                        ty: Box::new(Type::Reference(TypeReference {
                                            and_token: syn::token::And::default(),
                                            lifetime: None,
                                            mutability: None,
                                            elem: pat_type.ty.clone(),
                                        })),
                                    }))
                                }
                            }
                        },
                        _ => panic!("Not binding")
                    },

                _ => panic!("Not FnArg")
            }
        })
        .unzip()
}

fn env_input(f: &syn::TraitItemFn) -> bool {
    f.sig.inputs.first()
     .and_then(|a|
        match a {
            FnArg::Typed(pat_type) => {
                let mut ty = &*pat_type.ty;
                if let Type::Reference(TypeReference { elem, .. }) = ty {
                    ty = elem;
                }
                if let Type::Path(TypePath {
                    path: syn::Path { segments, .. },
                    ..
                }) = ty
                {
                    if segments.last().map_or(false, |s| s.ident == "Env") {
                        Some(())
                    } else {
                        None
                    }
                } else {
                    None
                }
            },

            FnArg::Receiver(_) => None,
        }
    ).is_some()
}

fn derive_client_impl(crate_path: &Path, name: &str, fns: &[syn::TraitItemFn]) -> TokenStream2 {
    let client_ident = format_ident!("{}", name);
    let impls: Vec<_> = fns.iter()
       .map(|f| {
           let fn_ident = &f.sig.ident;
           let fn_output = match &f.sig.output {
               syn::ReturnType::Default => quote!(()),
               syn::ReturnType::Type(_, typ) => quote!(#typ),
           };
           //let fn_try_output = f.try_output(crate_path);
           // taken from soroban-sdk-macros
           // Check for the Env argument.
           let (_, binds) = non_env_inputs(&f);
           match f.default {
               None =>
                   quote! {
                       #[allow(unused)]
                       pub fn #fn_ident(&self, #(#binds),*) -> #fn_output {
                           nondet::nondet()
                       }
                   },

               Some(ref default) => {
                   let def = default.clone();
                   quote! {
                       pub fn #fn_ident(&self, #(#binds),*) -> #fn_output {
                           let env = self.env.clone();
                           #def
                       }
                   }
               }
           }
    }).collect();
    quote! {
        pub struct #client_ident<'a> {
            pub env: #crate_path::Env,
            pub address: #crate_path::Address,
            #[doc(hidden)]
            _phantom: core::marker::PhantomData<&'a ()>,
        }

        impl<'a> #client_ident<'a> {
            pub fn new(env: &#crate_path::Env, address: &#crate_path::Address) -> Self {
                Self {
                    env: env.clone(),
                    address: address.clone(),
                    _phantom: core::marker::PhantomData,
                }
            }

            #(#impls)*
        }
    }
}

#[proc_macro_attribute]
pub fn cvt_contractclient(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match ContractClientArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let item_clone = proc_macro2::TokenStream::from(item.clone());

    let item_trait = parse_macro_input!(item as syn::ItemTrait);

    let methods: Vec<_> = item_trait
        .items
        .into_iter()
        .flat_map(|i| {
            match i {
                syn::TraitItem::Fn(f) => Some(f),
                _ => None
            }
        }).collect();

    let client_impl = derive_client_impl(&args.crate_path, &args.name, &methods);

    let stream = quote! {
        #item_clone

        #client_impl
    };

    stream.into()
}

fn of_named_fields(n: &Ident, named_fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let initialize = named_fields
        .named
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            quote! {
                #name: ::cvt_soroban::Nondet::nondet(),
            }
        });

    quote! {
        #n {
            #( #initialize )*
        }
    }
}

fn of_unnamed_fields(n: &Ident, unnamed: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let initialize = unnamed
        .unnamed
        .iter()
        .map(|f| {
            quote! {
                ::cvt_soroban::Nondet::nondet(),
            }
        });

    quote! {
        #n (
            #( #initialize )*
        )
    }
}

#[proc_macro_derive(Nondet)]
pub fn derive_nondet(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    match input.data {
        Enum(e) => {
            todo!("Enum not supported yet")
        },

        Union(u) => {
            todo!("Union not supported yet")
        },

        Struct(ds) => {
            match (ds.fields) {
                Fields::Unit => {
                    quote! {
                        impl ::cvt_soroban::Nondet for #name {
                            fn nondet() -> #name {
                                ()
                            }
                        }
                    }.into()
                },

                Named(named) => {
                    let init = of_named_fields(&name, &named);
                    quote! {
                        impl ::cvt_soroban::Nondet for #name {
                            fn nondet() -> #name {
                                #init
                            }
                        }
                    }.into()
                },

                Unnamed(fields) => {
                    let init = of_unnamed_fields(&name, &fields);
                    quote! {
                        impl ::cvt_soroban::Nondet for #name {
                            fn nondet() -> #name {
                                #init
                            }
                        }
                    }.into()
                }
            }
        }
    }
}
