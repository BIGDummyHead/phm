mod postman;
mod request_args;

use core::panic;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, FnArg, ItemFn, parse_macro_input, punctuated::Punctuated, token::Comma};

use crate::request_args::RequestArgs;
use postman::*;

/// # Get fn Input Names
///
/// Returns a `Vec` of TokenStreams of each argument name inside of an `&ItemFn`.
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

/// # route
///
/// Proc Macro Attribute used over functions that transform a route into a simplier API.
///
/// For example:
///
/// Using the route we can add routes via:
///
/// ```
///     # use phm::{App, HttpMethod::GET, app::{ClosedAppExt}, HttpRequest, Response, RequestError, Middleware};
///     # use phm_pm::route;
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
///     #[route]
///     async fn get_user(req: &mut HttpRequest<'_>, res: Response) -> Result<(), RequestError> {
///         res.status(200).text("user information");
///         Ok(())
///     }
///
///     // we can also expand upon this to define the route specific information
///     #[route(route="/api/user", method="POST", middleware(auth))]
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
///     # use phm_pm::route;
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
pub fn route(args: TokenStream, func_stream: TokenStream) -> TokenStream {
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

fn is_route(attr: &&Attribute) -> bool {
    attr.path()
        .segments
        .last()
        .expect("failed to get ending segment")
        .to_token_stream()
        .to_string()
        .trim()
        == "route"
}

fn get_request_attr(attrs: &Vec<Attribute>) -> Result<RequestArgs, darling::Error> {
    let request_attr = attrs
        .iter()
        .find(is_route)
        .ok_or(darling::Error::missing_field("missing route attribute"))?;
    RequestArgs::from_meta(&request_attr.meta)
}

/// # Postman
///
/// This macro allows you to document your API programatically by adding this with the `phm_pm::route` macro attribute.
///
/// At compile time a file is created called `postman/postman_api.json`. The file can then be imported into postman, which creates a collection with all collected routes.
///
/// ## Example
///
/// ```
///     # use phm::{App, HttpMethod::GET, app::{ClosedAppExt}, HttpRequest, Response, RequestError, Middleware};
///     # use phm_pm::{postman, route};
///     # use phm::web::{ArcMiddlewareClosure, middleware};
///
///     /// # post_user
///     ///Allows you to create a user!
///  #[postman]
///  #[route(route="/api/user", method="POST")]
///  async fn post_user(req: &mut HttpRequest<'_>, res: Response) -> Result<(), RequestError> {
///      res.status(201).text("created");
///      Ok(())
///  }
///
///     # fn main () {
///         # smol::block_on(async move {
///             # let app = App::bind("127.0.0.1:80").await.expect("failed to bind server");
///             # app.add_def(post_user).await.expect("");
///         # });
///     # }
///
///
/// ```
///
/// The follow postman schema is created:
///
/// ```json
/// ```
#[proc_macro_attribute]
pub fn postman(_args: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);

    let documentation: String = func
        .attrs
        .iter()
        .filter(|a| a.path().to_token_stream().to_string() == "doc")
        .map(|a| String::from_meta(&a.meta).expect("invalid doc"))
        .collect::<Vec<String>>()
        .join("\r\n");

    let request_args = get_request_attr(&func.attrs).expect("failed to parse request attribute");

    let req = Request::new(request_args.route, request_args.method, Some(documentation));
    let request_item = Item::create(func.sig.ident.to_string(), req);

    if request_args.module.is_empty() {
        add_to_schema(request_item);
    } else {
        let module_item = SCHEMA
            .write()
            .expect("could not read schem")
            .take_module(&request_args.module);

        // this is a request item, not a module
        let request_item = Box::new(request_item);

        if let Some(mut module) = module_item {
            module.item.push(request_item);
            add_to_schema(module); //re-add the module
        } else {
            let mut module_item = Item::folder(request_args.module);
            module_item.item.push(request_item);
            add_to_schema(module_item); // add the module
        }
    }

    func.to_token_stream().into()
}

#[proc_macro]
pub fn postman_info(input: TokenStream) -> TokenStream {
    let info = parse_macro_input!(input as Info);

    set_global_info(info);

    TokenStream::new()
}

#[proc_macro_attribute]
pub fn postman_module(_args: TokenStream, module: TokenStream) -> TokenStream {
    let module = parse_macro_input!(module as syn::ItemMod);

    let module_ident = &module.ident;

    let (_, module_items) = module
        .content
        .expect("This macro should only be used upon a blocked module.");

    let module_items: Vec<proc_macro2::TokenStream> = module_items
        .iter()
        .map(|mi| match mi {
            syn::Item::Fn(item_fn) => match get_request_attr(&item_fn.attrs) {
                Err(_) => item_fn.to_token_stream(),
                Ok(req_args) => {
                    // signature and block
                    let sig = &item_fn.sig;
                    let block = &item_fn.block;

                    //attributes
                    let attrs = item_fn.attrs.iter().filter(|f| !is_route(f));

                    let (route, method, middleware) = (
                        req_args.route,
                        req_args.method,
                        req_args.middleware,
                    );

                    let middleware_tokens = if middleware.is_empty() {
                        quote! {}
                    } else {
                        quote! {
                            , middleware(#(#middleware)*)
                        }
                    };

                    let module_ident_str = module_ident.to_string();

                    quote! {
                        #(#attrs)*
                        #[phm_pm::route(route=#route, method=#method, module=#module_ident_str #middleware_tokens)]
                        #sig #block
                    }
                }
            },
            _ => mi.to_token_stream(),
        })
        .collect();

    let vis = &module.vis;
    let ident = &module.ident;
    let attrs = &module.attrs;

    quote! {
        #(#attrs)*
        #vis mod #ident {
            #(#module_items)*
        }
    }
    .into()
}
