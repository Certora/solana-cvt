use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs};
use quote::ToTokens;


#[proc_macro_attribute]
/// This macro is used to insert a hook at the start of a function.
pub fn cvt_hook_start(attr: TokenStream, input: TokenStream) -> TokenStream {
    
    // parse the attribute argument
    let attr = parse_macro_input!(attr as AttributeArgs);
    if attr.len() != 1 {
        panic!("expected 1 argument");
    }

    let arg = &attr[0]; 

    // parse the input tokens and make sure it is a function
    let mut item: syn::Item = syn::parse(input).unwrap();
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn")
    };


    // insert tokens_start into fn item statements at position 0
    let tokens_start = quote! {
        if cfg!(feature = "certora") {
            #arg;
        }
    };
    fn_item.block.stmts.insert(0,syn::parse(tokens_start.into()).unwrap());

    item.into_token_stream().into()
}


#[proc_macro_attribute]
/// This macro is used to insert a hook at the end of a function.
pub fn cvt_hook_end(attr: TokenStream, input: TokenStream) -> TokenStream {

    // parse the attribute argument
    let attr = parse_macro_input!(attr as AttributeArgs);
    if attr.len() != 1 {
        panic!("expected 1 argument");
    }

    let arg = &attr[0]; 

    // parse the input tokens and make sure it is a function
    let mut item: syn::Item = syn::parse(input).unwrap();
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn")
    };

    // create tokens_end
    let tokens_end = quote! {
        if cfg!(feature = "certora") {
            #arg;
        }
    };

    // len of fn item statements
    let len = fn_item.block.stmts.len();

    // insert tokens_end into fn item statements at position len-1
    fn_item.block.stmts.insert(len-1,syn::parse(tokens_end.into()).unwrap());
    
    item.into_token_stream().into()
}
