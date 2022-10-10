use darling::FromMeta;
use proc_macro::TokenStream;
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
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let function = parse_macro_input!(input as ItemFn);

    let args = match MainMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let tokio = args.tokio.unwrap_or("mekena::tokio".to_string());

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

    let function_output = if let ReturnType::Type(_, t) = function.sig.output {
        *t
    } else {
        Type::Tuple(TypeTuple {
            paren_token: Paren::default(),
            elems: Punctuated::new(),
        })
    };

    let function_contents = function.block;

    quote! {
        // TODO: call tokio
        fn main() -> #function_output {
            let #system_name = mekena::system::System::new();

            #function_contents
        }
    }
    .into()
}
