use proc_macro::TokenStream;
use quote::{quote, ToTokens};

/// Service entry-point (main)
///
/// # Panics
///
/// Will panic if set not on a function
#[proc_macro_attribute]
pub fn svc_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("Invalid input");
    if let syn::Item::Fn(fn_item) = item {
        let block = &fn_item.block;
        let f = quote! {
            fn main() -> eva_common::EResult<()> {
                eva_sdk::service::svc_launch(eva_service_main)
            }
            async fn eva_service_main(
                    mut initial: eva_sdk::prelude::Initial) -> eva_common::EResult<()>
                #block
        };
        f.into_token_stream().into()
    } else {
        panic!("expected fn")
    }
}
