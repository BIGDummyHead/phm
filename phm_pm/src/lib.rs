use core::panic;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{FnArg, ItemFn, parse_macro_input, punctuated::Punctuated, token::Comma};

fn ensure_correct_args(inputs: &Punctuated<FnArg, Comma>) -> () {
    for arg in inputs {
        match arg {
            syn::FnArg::Receiver(_) => panic!("unexpected self argument"),
            syn::FnArg::Typed(pat_type) => {
                let type_name = pat_type.to_token_stream().to_string().to_lowercase();

                if type_name.contains("httprequest") || type_name.contains("response") {
                    continue;
                } else {
                    panic!("incorrect parameter");
                }
            }
        }
    }
}

#[proc_macro_attribute]
pub fn request(_attr: TokenStream, func_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(func_stream as ItemFn);

    // panics if incorrect arguments provided in the first function
    ensure_correct_args(&input.sig.inputs);

    let fn_name = input.sig.ident;

    let body = input.block;

    quote! {
        fn #fn_name<'a, 'b>(req: &'a mut phm::HttpRequest<'b>, res: &'a mut phm::Response)
         -> std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = Result<(), phm::RequestError>> + std::marker::Send + 'a>> {
            std::boxed::Box::new(async move #body)
        }
    }.into()
}
