#![crate_type = "proc-macro"]
#![allow(dead_code)]
#![allow(unused_variables)]

//extern crate proc_macro;
use std::{fmt::format, str::FromStr};
use quote::{quote};

use proc_macro::{token_stream, Literal, TokenStream};
use proc_macro2;
use syn::{token, DeriveInput, Expr, ItemStruct, LitStr, Stmt};
// use quote::quote;
// use syn::{parse_macro_input, Item,Expr};

// #[proc_macro]
// pub fn gen_crtran25_struct(_item: TokenStream) -> TokenStream{
//     let input = syn::parse_macro_input!(_item as syn::DeriveInput);
//     quote! { input}
// }
fn str_to_token(value:&str){
    println!("{}",value)
}


#[proc_macro]
pub fn make_answer(_item:TokenStream) -> TokenStream {
    // let ast = syn::parse_macro_input!(_item as syn::);
    eprintln!("input macro-> {:#?}",_item);
    // let ast = syn::parse_macro_input!(_item as proc_macro2::TokenStream);
    // eprintln!("syn ast-> {}",ast);
    proc_macro::TokenStream::new()
    // let ast:ItemStruct = syn::parse2(token2 ).unwrap();
    
    // println!("{:?}",&ast);
    // // let ident = ast.ident;
    // (quote! { 
    //         struct #ast.ident {a:i32,b:i32},
    //         let x = #ast.ident{a:1,b:2};
    //         x
    //      }).into()
    // TokenStream::from(expanded)
    // TokenStream::new();
    
    //funciona so com string literal
    //let strg = syn::parse_macro_input!(str as syn::LitStr  );
    // println!("{:?}",str);
    // let token = syn::parse_str::<Expr>(  &str).unwrap();
    // (quote::quote!{#token}).into()

}
