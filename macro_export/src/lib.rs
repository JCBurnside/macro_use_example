use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

#[proc_macro_attribute]
pub fn format(_meta: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let Signature {
        ident,
        inputs,
        output,
        ..
    } = input_fn.sig;
    let bool_error = if let ReturnType::Type(_, output) = output {
        if output.to_token_stream().to_string() != "bool" {
            Some(syn::Error::new(ident.span(), "Must return bool"))
        } else {
            None
        }
    } else {
        Some(syn::Error::new(ident.span(), "Must return bool"))
    };

    if inputs.len() != 1 {
        let mut err = syn::Error::new(ident.span(), "Must only accept a &str agrument");
        if let Some(inner) = bool_error {
            err.combine(inner);
        }
        return err
            .to_compile_error()
            .into();
    }

    if let Some(err) = bool_error {
        return err.to_compile_error().into();
    }

    let vis = &input_fn.vis;
    let body = &input_fn.block;
    let custom_t_name = ident.to_string() + "_t";
    TokenStream::from(quote! {
        #vis struct #custom_t_name;

        impl macro_support::CustomTrait for #custom_t_name
        {
            const NAME : &'static str = "#ident";
            fn is_valid(&self,:&str) -> bool {
                #body
            }
        }

        const #ident : #custom_t_name = #custom_t_name {};

        impl ToString for #custom_t_name {
            fn to_string(&self) -> String {
                concat!("format: ", <Self as CustomValidator>::NAME).to_string()
            }
        }
    })
}
