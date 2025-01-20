use core::fmt;
use std::fmt::{write, Display};

use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{Error, Visitor};

use crate::config::{Empty, EmptyGeneric, Generator, KeyOfGeneric, Property, PropertyWithDefault};

#[derive(Debug, Serialize, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Licenses {
    MIT,
    APACHE2_0,
    GPL3,
    GPL2,
    BSD3,
    BSD2,
    CCO,
    MPL2,
    EPL,
    AGPL3,
    UNKNOWN
}

impl Display for Licenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Licenses::MIT => write!(f, "MIT"),
            Licenses::APACHE2_0 => write!(f, "Apache 2.0"),
            Licenses::GPL3 => write!(f, "GPL 3.0"),
            Licenses::GPL2 => write!(f, "GPL 2.0"),
            Licenses::BSD3 => write!(f, "BSD 3.0"),
            Licenses::BSD2 => write!{f, "BSD 2.0"},
            Licenses::CCO => write!(f, "CCO"),
            Licenses::MPL2 => write!(f, "MPL 2.0"),
            Licenses::EPL => write!(f, "EPL"),
            Licenses::AGPL3 => write!(f, "AGPL 3.0"),
            Licenses::UNKNOWN => write!(f, "Custom")
        }
    }
}

impl Licenses {
    pub fn to_enum(value: &str) -> Self {
        match value {
            "MIT" => Licenses::MIT,
            "Apache 2.0" => Licenses::APACHE2_0,
            "GPL 3.0" => Licenses::GPL3,
            "GPL 2.0" => Licenses::GPL2,
            "BSD 3.0" => Licenses::BSD3,
            "BSD 2.0" => Licenses::BSD2,
            "CCO" => Licenses::CCO,
            "MPL 2.O" => Licenses::MPL2,
            "EPL" => Licenses::EPL,
            "AGPL 3.0" => Licenses::AGPL3,
            _ => Licenses::UNKNOWN
        }
    }
}


// Deserialization function for 'publisher' field
fn map_publisher<'de, D>(deserializer: D) -> Result<Option<Property<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    Ok(value.map(|v| Property {
        key: "publisher".to_string(),
        value: Some(v),
    }))
}

fn map_name<'de, D>(deserializer: D) -> Result<Option<Property<Licenses>>, D::Error>
where
    D: Deserializer<'de>,
{
    // First, deserialize the input value as an Option<String>
    let value: Option<String> = Option::deserialize(deserializer)?;

    Ok(match value {
        Some(v) => {
            Some(Property {
                key: "name".to_string(),
                value: Some(Licenses::to_enum(&v)),
            })
        }
        None => None, // Return None if the value is None
    })
}


fn map_location<'de, D>(deserializer: D) -> Result<Option<PropertyWithDefault<String>>, D::Error>
where 
    D: Deserializer<'de>
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    // If the value is `Some`, return a `Property` with key and value wrapped in `Some`
    // If the value is `None`, return `None`
    Ok(match value {
        Some(v) => {
            Some(PropertyWithDefault {
                key: "location".to_string(),
                value: v
            })
        },
        None => Some(PropertyWithDefault {
            key: "location".to_string(),
            value: "root directory".to_string()
        }),
    })
}

// Default function for 'location' field
fn default_location() -> Option<PropertyWithDefault<String>> {
    Some(PropertyWithDefault {
        key: "location".to_string(),
        value: "root directory".to_string(),
    })
}

// Default function for 'license' field
pub fn default_license() -> Option<Property<License>> {
    None
}

pub fn deserialize_license<'de, D>(deserializer: D) -> Result<Option<Property<License>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<License> = Option::deserialize(deserializer)?;
    // If the value is `Some`, return a `Property` with key and value wrapped in `Some`
    // If the value is `None`, return `None`
    Ok(match value {
        Some(v) => {
            if v.name.is_some() || v.publisher.is_some() { // if both name and publisher are missing we can't create the license
                Some(Property {
                    key: "license".to_string(),
                    value: Some(v)
                })
            } else {
                None
            }
        },
        None => None,
    })
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct License {
    #[serde(default, deserialize_with = "map_publisher")]
    pub publisher: Option<Property<String>>,
    #[serde(default, deserialize_with = "map_name")]
    pub name: Option<Property<Licenses>>,
    #[serde(default = "default_location", deserialize_with = "map_location")]
    pub location: Option<PropertyWithDefault>,
}

impl License {
    pub fn new(publisher: Option<&str>, name: Option<Licenses>, location: Option<&str>) -> Self {
        Self {
            publisher: publisher.map(|p| Property {
                key: "publisher".to_string(),
                value: Some(p.to_string()),
            }),
            name: name.map(|n| Property {
                key: "name".to_string(),
                value: Some(n)
            }),
            location: location.map_or_else(|| Some(PropertyWithDefault {
                key: "location".to_string(),
                value: "root directory".to_string()
            }), |value| Some(PropertyWithDefault {
                key: "location".to_string(),
                value: value.to_string()
            }))
        }
    }
}

impl EmptyGeneric for License {
    fn empty() -> Self {
        Self {
            publisher: None,
            name: None,
            location: None
        }
    }
}

impl KeyOfGeneric for License {
    fn get_key(&self) -> String {
        return "license".to_string();
    }
}

impl Generator<License> for Property<License> {
    fn generate_comment(license: &Property<License>) {
        let final_comment: &str;

        let publisher = &license.value.as_ref().unwrap().publisher;
        let name = &license.value.as_ref().unwrap().name;
        let mut location = &license.value.as_ref().unwrap().location;

        if publisher.is_none() || name.is_none() {return }; // if publisher or name are missing we can't generate the comment

        // if location.is_none() {
        //     location =  Some(PropertyWithDefault::on_none("location", "root directory".to_string()));
        // }

    }
}