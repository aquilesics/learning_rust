use proc_macro2::TokenStream;
use std::str::FromStr;
use syn::{Field, ItemStruct};

#[derive(Debug)]
pub struct CustomField {
    pub field: syn::Ident,
    pub typ: syn::Ident,
}

#[derive(Debug)]
pub struct StructTemplate {
    pub struct_name: syn::Ident,
    pub custom_fields: Vec<CustomField>,
}

///retorna ast de um codigo valido Rust
pub fn str_to_ast(code: &str) -> syn::ItemStruct {
    let token2 = proc_macro2::TokenStream::from_str(&code).unwrap();
    let ast: ItemStruct = syn::parse2(token2).unwrap();
    ast
}

impl StructTemplate {
    //cria um template valido de uma struct de uma ast
    pub fn struct_from_ast(ast: ItemStruct) -> StructTemplate {
        let struct_name = ast.ident;
        let fields = ast.fields;
        match fields {
            syn::Fields::Named(x) => {
                let mut custom_fields: Vec<CustomField> = Vec::new();
                for _field in x.named {
                    if let Some(field) = _field.ident {
                        if let syn::Type::Path(_path) = _field.ty {
                            let typ = _path.path.segments[0].ident.clone();
                            custom_fields.push(CustomField { field, typ });
                        }
                    };
                }
                StructTemplate {
                    struct_name,
                    custom_fields,
                }
            }
            _ => panic!("error on parser struct"),
        }
    }
}
macro_rules! define_struct {
    ( $name:ident, $($($field:ident,$typ:ty),*),*) => {
        struct $name { $($($field:ident,$typ:ty),*),*}
    };
}

macro_rules! create_struct2 {
    ($name:ident($(($field:ident,$t:ty)),*)) => {
            #[derive(Debug)]
            struct $name { $($field :$t),* }
        
    };
}

pub fn create_struct_from_template(st: StructTemplate) {
    let name = st.struct_name;
    let fields = st
        .custom_fields
        .into_iter()
        .map(|x| (x.field, x.typ))
        .collect::<Vec<(syn::Ident, syn::Ident)>>();

    // define_struct!(name,fields)
}
