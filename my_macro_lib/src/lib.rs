use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn compute_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // The parse_macro_input! macro is used to parse the input tokens into an ItemFn, which represents a function.
    let input = parse_macro_input!(item as ItemFn);
    // fn_name: The name of the function.
    // fn_body: The body of the function.
    // fn_visibility: The visibility (e.g., pub).
    // fn_signature: The function signature.
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_visibility = &input.vis;
    let fn_signature = &input.sig;

    let expanded = quote! {
        #fn_visibility #fn_signature {
            let start = std::time::Instant::now();

            #fn_body

            let elapsed = start.elapsed();
            println!("{} took {:?}", stringify!(#fn_name), elapsed);
        }
    };

    expanded.into()
}
