use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;

#[proc_macro_hack]
pub fn styles(input: TokenStream) -> TokenStream {

    let src = input.to_string().replace(" ", "");
    let src = &src[..];

    quote!(style::Stylesheet::parse(
        #src
    ))
    .into()
}
