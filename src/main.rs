use ::std::fs::{read_to_string, File};
use std::str::FromStr;
use apache_avro::schema;
use apache_avro::types::Value;
use apache_avro::{from_value, types::Record, Codec, Reader, Schema, Writer};
use macros::make_answer;
use proc_macro2::TokenStream;
use syn::token::Struct;
use std::any::type_name_of_val;
use std::collections::btree_set::Union;
use std::collections::HashMap;
use std::fmt::DebugStruct;
use std::time::Instant;
extern crate macros;
use syn::{self, token, Expr, ItemStruct};
// use macros::gen_crtran25_struct;
use quote::{ quote};


fn read_lines(path: &str) -> Result<(), apache_avro::Error> {
    let mut counter = 0;

    let file = File::open(path).unwrap();
    let reader = Reader::new(file);

    if let Ok(table) = reader {
        let s = table.writer_schema();

        match s {
            Schema::Record(r) => {
                let mut fields_and_types = Vec::new();
                for i in &r.fields{
                    let key = &i.name;
                    let val;
                    match &i.schema {
                        Schema::Union(u) => {
                            match u.variants()[1] {     
                                Schema::Double => {val = "f64"},
                                Schema::Float => {val = "f64"},
                                Schema::Int =>{val = "i64"},
                                Schema::String =>{val = "String"},
                                _ => panic!("Error on parse data type: {}",type_name_of_val(u))
                            }
                        },
                        _ =>panic!("error on unwrap schema type")                        
                    }
                    let key_val_strg = format!("{}:{}",key,val);
                    fields_and_types.push(key_val_strg)
                };
                let struct_repr = format!("struct CRTRAN25{{ {} }}",fields_and_types.join(",\n"));
                println!("{}",struct_repr);
            }
            _=> println!("error on load schema")
        };
    };
    Ok(())
}

pub fn main() {
    // let started = Instant::now();
    // for i in 0..1 {
    //     let path = format!(r"C:\Users\xj\repo\learning_rust\data_avro\test_{}.avro", i);
    //     let _ = read_lines(&path);
    // }
    // let ended = started.elapsed().as_secs();

    // println!("levou {} secs", ended);
    

    // macro_rules! test {
    //     ($name:ident,($field:tt : $type:ty))=> {
    //         // gen_crtran25_struct!( struct $name {$field:$type});
    //         struct $name {$field:$type}
    //     };
    // }
    let code =   "assert_eq!(1,2)" ;
    let ast = syn::parse_str::<Expr>(&code).unwrap();
    println!("{:?}\n\n",ast);
    let tok:TokenStream = quote!{#ast}.into();
    println!("{:?}",tok);

    let a = make_answer!( "assert_eq!(1,1)" );

    println!("{:?}",a)
    
    

    
 

}
