use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};


/// Mark a method as a CERTORA rule
///
/// # Example
///
/// ```
/// use certora_macros::rule;
/// use cvt::CERTORA_assert;
/// #[rule]
/// fn foo()  {
///    cvt::CERTORA_assert(false);
/// }
/// ```
#[proc_macro_attribute]
pub fn rule(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut fn_ast = parse_macro_input!(input as ItemFn);
    // add #[no_mangle] attribute
    fn_ast.attrs.push(parse_quote!(#[no_mangle]));
    TokenStream::from(quote!(#fn_ast))
}
