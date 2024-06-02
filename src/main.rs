#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use ::std::fs::{read_to_string, File};
use apache_avro::schema::{self, Name};
use apache_avro::types::Value;
use apache_avro::{from_value, types::Record, Codec, Reader, Schema, Writer};
use csv::Error;
use macros::make_answer;
use proc_macro2::TokenStream;
use std::any::type_name_of_val;
use std::collections::btree_map::Keys;
use std::collections::btree_set::Union;
use std::collections::HashMap;
use std::env::temp_dir;
use std::ffi::OsStr;
use std::fmt::DebugStruct;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;
use std::{clone, fs};
use syn::parse::Parse;
use syn::token::Struct;
use toml::value::Date;
extern crate macros;
extern crate parser;
use syn::{self, parse, token, Expr, FieldsNamed, Ident, ItemStruct, Stmt};
// use macros::gen_crtran25_struct;
use proc_macro2;
use quote::quote;
use serde::Deserialize;
use toml;
pub mod utils;

struct DataTable<'a> {
    src: &'a str,
    table: Reader<'a, File>,
    partitions: std::vec::IntoIter<PathBuf>,
    current_partition: PathBuf,
}

impl DataTable<'_> {
    fn new(src: &str) -> DataTable<'_> {
        let mut part = DataTable::get_avro_files(src);
        let curr = part.next().unwrap();
        DataTable {
            src,
            partitions: part,
            table: DataTable::get_table(curr.clone()),
            current_partition: curr.clone(),
        }
    }
    fn next_table(&mut self) -> Option<()> {
        if let Some(part) = self.partitions.next() {
            self.current_partition = part.clone();
            self.table = DataTable::get_table(part);
            Some(())
        } else {
            None
        }
    }
    fn get_table(partition: PathBuf) -> Reader<'static, File> {
        let file = File::open(partition).unwrap();
        let reader = Reader::new(file);
        if let Ok(table) = reader {
            table
        } else {
            panic!("error on create table from avro file")
        }
    }
    fn get_avro_files(src: &str) -> std::vec::IntoIter<PathBuf> {
        let folder = std::path::Path::new(src);
        let mut avro_files: Vec<PathBuf> = Vec::new();
        for file in folder.read_dir().expect("error on iter over path") {
            if let Ok(_file) = file {
                let is_file = _file.path().is_file();
                if is_file && _file.path().extension().unwrap() == "avro" {
                    let path = _file.path();
                    avro_files.push(path);
                }
            } else {
                panic!("error on get DirEntry from path")
            }
        }
        avro_files.into_iter()
    }
}

fn record_into_hsmap(row: Result<Value, apache_avro::Error>) -> HashMap<String, Box<Value>> {
    if let Ok(row) = row {
        if let Value::Record(row) = row {
            let mut hs = HashMap::new();
            for column in row {
                match column.1 {
                    Value::Union(i, val) => {
                        hs.insert(column.0, val);
                    }
                    _ => panic!("erro ao trasformar row em hashmap"),
                }
            }
            hs
        } else {
            panic!("row nao eh do tipo record");
        }
    } else {
        panic!("erro, row do tipo none")
    }
}

pub fn main() {
    let started = Instant::now();

    let config = utils::load_settings("./config.toml").unwrap();
    //print!("{:#?}", config);

    let data_table = DataTable::new(&config.input_data.auth_abs_path.as_str());

    let mut iter_dt = data_table.table.into_iter();
    //let mut x = 0;
    let mut udv = HashMap::new();
    while let Some(row) = iter_dt.next() {
        let hs = record_into_hsmap(row);
        let k = (**hs.get("str_col2").unwrap()).clone();
        match &k {
            Value::String(s) => {
                
                udv.entry(s.clone())
                    .and_modify(|x| *x += 1)
                    .or_insert(0);
            }
            _ => {}
        }

        //  udv.entry(k).or_insert(0);
    }
    print!("\r{:#?}",udv.get("H"));
    // for i in 0..1 {
    //     let path = format!(r"C:\Users\xj\repo\learning_rust\data_avro\test_{}.avro", i);
    //     let _ = read_lines(&path);
    // }
    let ended = started.elapsed().as_secs();

    println!("\nlevou {} secs", ended);
}
