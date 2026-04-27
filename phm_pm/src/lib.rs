mod request_args;
mod route_document;

use core::panic;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{FnArg, ItemFn, parse_macro_input, punctuated::Punctuated, token::Comma};

use crate::request_args::RequestArgs;

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

fn get_fn_input_names(input: &ItemFn) -> Vec<proc_macro2::TokenStream> {
    input
        .sig
        .inputs
        .iter()
        .map(|i| match i {
            FnArg::Receiver(_) => panic!("unsupported self argument"),
            FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                    let arg_name = &pat_ident.ident;
                    quote! { #arg_name }
                } else {
                    panic!("could not discover identifier");
                }
            }
        })
        .collect()
}

/// ## Dry Request
///
/// Allows for a base request fn to be made without external metadata.
fn dry_request(input: ItemFn) -> TokenStream {
    let mut names = get_fn_input_names(&input);

    let fn_name = input.sig.ident;
    let body = input.block;

    if names.len() != 2 {
        panic!("invalid input captured");
    }

    let req_name = names.remove(0);
    let res_name = names.remove(0);

    quote! {
        fn #fn_name<'a, 'b>(#req_name: &'a mut phm::HttpRequest<'b>, #res_name: &'a mut phm::Response)
         -> std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = Result<(), phm::RequestError>> + std::marker::Send + 'a>> {
            std::boxed::Box::pin(async move #body)
        }
    }.into()
}

/// # request
///
/// Proc Macro Attribute used over functions that transform a request into a simplier API.
///
/// For example:
///
/// Using the request we can add routes via:
///
/// ```
///     # use phm::{App, HttpMethod::GET, app::{ClosedAppExt}, HttpRequest, Response, RequestError, Middleware};
///     # use phm_pm::request;
///     # use phm::web::{ArcMiddlewareClosure, middleware};
/// 
///     fn auth() -> ArcMiddlewareClosure {
///         middleware(|req, res| {
///             Box::pin(async move {
///                 Middleware::Next
///             })
///         })
///     }
///
///     #[request]
///     async fn get_user(req: &mut HttpRequest<'_>, res: Response) -> Result<(), RequestError> {
///         res.status(200).text("user information");
///         Ok(())
///     }
/// 
///     // we can also expand upon this to define the route specific information
///     #[request(route="/api/user", method="POST", middleware(auth))]
///     async fn post_user(req: &mut HttpRequest<'_>, res: Response) -> Result<(), RequestError> {
///         res.status(201).text("created");
///         Ok(())
///     }
///
///     # fn main () {
///         # smol::block_on(async move {
///     // bind app, etc...
///     let app = App::bind("127.0.0.1:80").await.expect("failed to bind server");
///     app.add_route(GET, "/users", vec![], get_user).await.expect("");
///     app.add_def(post_user).await.expect("");
///         # });
///     # }
///
///
/// ```
///
/// As opposed:
///```
///     # use phm::{App, HttpMethod::GET, HttpRequest, Response, RequestError};
///     # use phm_pm::request;
///
///
///     # fn main () {
///         # smol::block_on(async move {
///     // bind app, etc...
///     let app = App::bind("127.0.0.1:80").await.expect("failed to bind server");
///     app.add_route(GET, "/users", vec![], |req, res| {
///         Box::pin(async move {
///             Ok(())
///         })
///     }).await.expect("");
///         # });
///     # }
/// ```
///
///
#[proc_macro_attribute]
pub fn request(args: TokenStream, func_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(func_stream as ItemFn);

    // panics if incorrect arguments provided in the first function
    ensure_correct_args(&input.sig.inputs);

    if args.is_empty() {
        return dry_request(input);
    }

    let mut names = get_fn_input_names(&input);

    if names.len() != 2 {
        panic!("invalid input captured");
    }

    let req_name = names.remove(0);
    let res_name = names.remove(0);

    // parse the arguments, ensuring they are of request args
    let attr_args =
        darling::ast::NestedMeta::parse_meta_list(args.into()).expect("failed to parse meta list");
    let request_meta = RequestArgs::from_list(&attr_args).expect("failed to parse request meta");

    let route = request_meta.route.leak();
    let method = request_meta.method;

    let fn_name = input.sig.ident;
    let body = input.block;

    let middleware: Vec<_> = request_meta.middleware.iter().collect();

    quote! {
        fn #fn_name()
         -> phm::app::RouteDefinition {

            use phm::middleware;

                let middleware_clones = vec![#(#middleware()), *];
                phm::app::RouteDefinition::new(#route, #method.to_string(), middleware_clones, std::boxed::Box::new(|#req_name: &mut phm::HttpRequest<'_>, #res_name: &mut phm::Response| {
                    std::boxed::Box::pin(async move #body)
                }))
        }
    }.into()
}
