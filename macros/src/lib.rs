#![crate_type = "proc-macro"]
//extern crate proc_macro;
use std::str::FromStr;
use quote::{self, ToTokens};

use proc_macro::TokenStream;
use syn::{token, DeriveInput, LitStr,Expr};
// use quote::quote;
// use syn::{parse_macro_input, Item,Expr};

// #[proc_macro]
// pub fn gen_crtran25_struct(_item: TokenStream) -> TokenStream{
//     let input = syn::parse_macro_input!(_item as syn::DeriveInput);
//     quote! { input}
// }



#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    // _item.to_string().parse().unwrap();
    let strg = syn::parse_macro_input!(_item as LitStr);
    let token = syn::parse_str::<Expr>(  &strg.value() ).unwrap();

    (quote::quote!{#token}).into()

}
