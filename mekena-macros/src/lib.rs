use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Paren, AttributeArgs, FnArg, ItemFn, Pat,
    ReturnType, Type, TypeTuple,
};

#[derive(Debug, FromMeta)]
struct MainMacroArgs {
    #[darling(default)]
    tokio: Option<String>,
}

/// The `mekena::main` macro, meant to be called on the main function of a program.
#[proc_macro_attribute]
pub fn main(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let function = parse_macro_input!(input as ItemFn);

    let args = match MainMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return proc_macro::TokenStream::from(e.write_errors());
        }
    };

    let tokio: TokenStream = args
        .tokio
        .as_deref()
        .unwrap_or("mekena::tokio")
        .parse()
        .unwrap();

    let _asyncness = function
        .sig
        .asyncness
        .expect("Could not find `async` marker.");

    let function_output = if let ReturnType::Type(_, t) = function.sig.output {
        *t
    } else {
        Type::Tuple(TypeTuple {
            paren_token: Paren::default(),
            elems: Punctuated::new(),
        })
    };

    let system_name = {
        if let Some(first) = function.sig.inputs.first() {
            if let FnArg::Typed(pattern_type) = first {
                if let Pat::Ident(identifier) = &*pattern_type.pat {
                    &identifier.ident
                } else {
                    panic!("could not find system variable name")
                }
            } else {
                panic!("could not find system variable pattern")
            }
        } else {
            panic!("could not find system variable in fn sig");
        }
    };

    let function_contents = function.block;

    quote! {
        #[#tokio::main]
        async fn main() -> #function_output {
            // You should be bringing `System` into scope yourself.`
            let mut #system_name = System::new();
            #function_contents
        }
    }
    .into()
}
