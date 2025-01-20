// src/test/config.rs

use crate::config;  // Import the `config` module (from `lib.rs`)

#[cfg(test)]
mod tests {
    use crate::generators::license::Licenses;

    use super::*;  // Brings everything from the parent module (`config.rs`)
    use std::fs::{File, remove_file};
    use std::io::{Read, Write};

    fn generate_sample_config() -> String {
        r#"{
            "location": {
                "publisher": "Meta",
                "license_name": "MIT",
                "license_location": "root directory"
            }
        }"#.to_string()
    }
}
