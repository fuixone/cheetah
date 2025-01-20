use std::{fs::File};
use std::io::Read;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;

use crate::generators::license::{License, default_license, deserialize_license};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Property<T = String> {
    pub(crate) key: String,
    pub value: Option<T>
}

impl<T> Property<T> {
    pub fn new(key: &str, value: Option<T>) -> Self {
        Self {
            key: key.to_string(),
            value
        }
    }
}

pub trait KeyOfGeneric {
    fn get_key(&self) -> String;
}

// Implement KeyOfGeneric for Property<T> when T implements KeyOfGeneric
impl<T: KeyOfGeneric> KeyOfGeneric for Property<T> {
    fn get_key(&self) -> String {
        self.key.clone() // Assuming you want to return the key stored in Property
    }
}

pub trait Empty<T> {
    fn empty(generic: T) -> Self;
}

pub trait EmptyGeneric {
    fn empty() -> Self;
}

impl<T: EmptyGeneric + KeyOfGeneric> Empty<T> for Property<T> {
    fn empty(generic: T) -> Self {
        Self {
            key: generic.get_key(),
            value: Some(T::empty())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PropertyWithDefault<T = String> {
    pub key: String,
    pub value: T
}

impl<T> PropertyWithDefault<T> {
    pub fn on_none(key: &str, value: T) -> Self {
        Self {
            key: key.to_string(),
            value
        }
    }
}

pub trait Generator<T> {
    fn generate_comment(property: &Property<T>);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_license", deserialize_with = "deserialize_license")]
    pub license: Option<Property<License>>
}

impl Config {
    pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {

     let mut file = File::open(file_path)?;

     let mut content = String::new();
     file.read_to_string(&mut content)?;

     let config: Config = serde_json::from_str(&content)?;

     Ok(config)

    }
}