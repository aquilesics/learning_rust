use csv::Error;
use  serde::Deserialize;
use toml;
use std::fs::File;
use std::io::{self, Read} ;

#[derive(Deserialize,Debug)]
pub struct Config{
    title:String,
    pub input_data:InputPath
}

#[derive(Deserialize,Debug)]
pub struct InputPath{
    pub auth_abs_path:String
}

#[macro_export]
macro_rules! pprint {
    ($($item:tt),*) => {
        $( println!("{:#?}\n",$item); )*
    };
}

pub fn load_settings(file:&str) -> Result<Config,Error >{
    if let Ok(mut file) = File::open(&file){
            let mut buff = String::new();
            file.read_to_string(&mut buff).unwrap();
            let config:Config = toml::from_str(&buff).unwrap();
            Ok(config)
    }
    else{
        panic!("erro ao ler arquivo de configuracao")
    }
}
